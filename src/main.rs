extern crate ev3dev_lang_rust;

use ctrlc;
use tokio;

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::GyroSensor;
use ev3dev_lang_rust::Ev3Result;

use robocode::{Bot, MoveOptions, Wheels};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

use uom::si::f32::Length;
use uom::si::length::{inch, meter};

#[tokio::main]
async fn main() -> Ev3Result<()> {
	let running = Arc::new(AtomicBool::new(true));
	let r = running.clone();
	ctrlc::set_handler(move || {
		r.store(false, Ordering::SeqCst);
	})
	.unwrap();

	let bot = Bot {
		wheels: Mutex::new(Wheels {
			motors: (
				LargeMotor::get(MotorPort::OutB)?,
				LargeMotor::get(MotorPort::OutC)?,
			),
			radius: Length::new::<meter>(0.02667),
		}),
		medium: Mutex::new(Some(MediumMotor::find()?)),
		gyro: Some(GyroSensor::find()?),
		ultrasonic: None,
		running,
	};

	let move_options = MoveOptions {
		forward: true,
		speed: 60,
	};

	let motor = MediumMotor::find()?;
	tokio::spawn(async move {
		for _ in 0..10 {
			griddy(&motor).unwrap();
		}
	});

	bot.move_distance(&move_options, Length::new::<inch>(18.0))?;
	bot.stop_movement()?;
	Ok(())
}

fn griddy(motor: &MediumMotor) -> Ev3Result<()> {
	motor.set_duty_cycle_sp(100)?;
	motor.run_direct()?;

	std::thread::sleep(std::time::Duration::from_millis(100));

	motor.set_duty_cycle_sp(-100)?;
	motor.run_direct()?;

	std::thread::sleep(std::time::Duration::from_millis(100));

	Ok(())
}
