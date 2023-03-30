extern crate ev3dev_lang_rust;

use ctrlc;
use ev3dev_lang_rust::motors::{LargeMotor, MotorPort, MediumMotor};
use ev3dev_lang_rust::sensors::{GyroSensor, UltrasonicSensor};
use ev3dev_lang_rust::sound;
use ev3dev_lang_rust::Ev3Result;
use uom::si::f32::Length;
use uom::si::length::{centimeter, meter, inch, foot};
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
        speed: 30,
    };
    let back_options = MoveOptions {
        forward: false,
        speed: 30,
    };

    let turn_options = TurnOptions {
        speed: 20,
        direction: Direction::RIGHT
    };


    // move 50 cm
    bot.move_distance(&move_options, Length::new::<centimeter>(50.0))?;
    // 90 degree turn
    bot.turn_angle(&turn_options, 90)?;


    // sensabot
    let arm = MediumMotor::get(MotorPort::OutA)?;
    for _ in 0..3 {
        bot.move_distance(&move_options, Length::new::<foot>(1.0))?;
        arm.set_duty_cycle_sp(-40)?;
        arm.run_direct()?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        arm.set_duty_cycle_sp(40)?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        arm.stop()?;
    }

    // dizzy drill
    for _ in 0..3 {
        bot.move_distance(&move_options, Length::new::<centimeter>(50.0))?;
        bot.turn_angle(&turn_options, 90)?;
    }

    // orchard
    let do_row = || {
        // clear one side
        bot.move_distance(&move_options, Length::new::<centimeter>(30.0)).unwrap();

        // get to other side
        bot.turn_angle(&turn_options, 90).unwrap();
        bot.move_distance(&move_options, Length::new::<centimeter>(10.0)).unwrap();
        bot.turn_angle(&turn_options, 90).unwrap();

        // clear other side
        bot.move_distance(&move_options, Length::new::<centimeter>(30.0)).unwrap();

        // reset
        bot.turn_angle(&turn_options, 90).unwrap();
        bot.move_distance(&move_options, Length::new::<centimeter>(10.0)).unwrap();
        bot.turn_angle(&turn_options, 90).unwrap();
    };

    // first row
    do_row();

    // move to 2nd row
    bot.turn_angle(&turn_options, 90).unwrap();
    bot.move_distance(&move_options, Length::new::<centimeter>(20.0)).unwrap();
    bot.turn_angle(&TurnOptions { speed: 20, direction: Direction::LEFT }, 90).unwrap();
    do_row();

    // move to 3rd row
    bot.turn_angle(&turn_options, 90).unwrap();
    bot.move_distance(&move_options, Length::new::<centimeter>(20.0)).unwrap();
    bot.turn_angle(&TurnOptions { speed: 20, direction: Direction::LEFT }, 45).unwrap();
    do_row();

	// bot.turn_angle(&TurnOptions{direction: Direction::RIGHT, speed: 20}, 90)?;
    // bot.move_distance(&move_options, Length::new::<centimeter>(50.0))?; bot.turn_angle(&TurnOptions{direction: Direction::RIGHT, speed: 20}, 90)?;
    // bot.move_distance(&move_options, Length::new::<centimeter>(50.0))?;

    // bot.move_distance(&move_options, Length::new::<inch>(12.0))?;
    
    //motor.stop()?;
    //bot.move_distance(&MoveOptions { forward: false, speed: 40}, Length::new::<centimeter>(50.0))?;

    Ok(())
}
