extern crate ev3dev_lang_rust;

use tokio;
use ctrlc;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort, MediumMotor};
use ev3dev_lang_rust::sensors::{GyroSensor, UltrasonicSensor};
use ev3dev_lang_rust::sound;
use ev3dev_lang_rust::Ev3Result;
use uom::si::f32::Length;
use uom::si::length::{centimeter, meter, inch};
use robocode::{Bot, Direction, MoveOptions, TurnOptions, Wheels};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Ev3Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    });

    let bot = Bot {
        wheels: Wheels {
            motors: (LargeMotor::get(MotorPort::OutB)?, LargeMotor::get(MotorPort::OutC)?),
            radius: Length::new::<meter>(0.02667)
        },
        medium: Some(MediumMotor::find()?),
        gyro: Some(GyroSensor::find()?),
        ultrasonic: None,
        running: running.clone(),
    };

    let move_options = MoveOptions {
        forward: true,
        speed: 60,
    };
    let back_options = MoveOptions {
        forward: false,
        speed: 40,
    };

    bot.move_distance(&move_options, Length::new::<inch>(18.0))?;
    bot.stop_movement()?;
    Ok(())
}
