extern crate ev3dev_lang_rust;

use ctrlc;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{GyroSensor, UltrasonicSensor};
use ev3dev_lang_rust::sound;
use ev3dev_lang_rust::Ev3Result;
use uom::si::f32::Length;
use uom::si::length::{centimeter, meter};
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
        ultrasonic: UltrasonicSensor::find()?,
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
    let turn_options = TurnOptions {
        speed: 60,
        direction: Direction::LEFT,
    };
    while running.load(Ordering::SeqCst) {
        bot.turn_until(&turn_options, || {
            Ok(bot.get_distance_from_obstacle()? > Length::new::<centimeter>(10.0))
        })?;
        bot.move_distance(&back_options, Length::new::<centimeter>(20.0))?;
        bot.move_until(&move_options, || {
            Ok(bot.get_distance_from_obstacle()? <= Length::new::<centimeter>(10.0))
        })?;
    }

    bot.stop_movement()?;
    Ok(())
}
