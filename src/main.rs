extern crate ev3dev_lang_rust;

use ctrlc;

use ev3dev_lang_rust::motors::{LargeMotor, MediumMotor, MotorPort};
use ev3dev_lang_rust::sensors::GyroSensor;
use ev3dev_lang_rust::Ev3Result;

use robocode::{Bot, MoveOptions, Wheels};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

use uom::si::f32::Length;
use uom::si::length::{inch, meter, centimeter, foot};

fn main() -> Ev3Result<()> {
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

    bot.move_distance(&move_options, Length::new::<centimeter>(50.0))?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    bot.turn_angle(&robocode::TurnOptions { speed: 20, direction: robocode::Direction::RIGHT}, 90)?;


	Ok(())
}
