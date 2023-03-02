extern crate ev3dev_lang_rust;

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

fn main() -> Ev3Result<()> {
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
        gyro: GyroSensor::find()?,
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
    bot.move_distance(&move_options, Length::new::<inch>(24.0))?;
    bot.turn_angle(&TurnOptions {speed: 20, direction: robocode::Direction::RIGHT}, 90)?;
    bot.move_distance(&move_options, Length::new::<inch>(12.0))?;
    let motor = MediumMotor::get(MotorPort::OutA)?;
    motor.set_duty_cycle_sp(-40)?;
    bot.move_distance(&MoveOptions { forward: false, speed: 20 }, Length::new::<inch>(20.0))?;

    Ok(())
}
