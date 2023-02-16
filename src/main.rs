extern crate ev3dev_lang_rust;


use ev3dev_lang_rust::motors::LargeMotor;
use ev3dev_lang_rust::motors::MotorPort;
use ev3dev_lang_rust::sensors::GyroSensor;
use ev3dev_lang_rust::Ev3Result;
use uom::si::f32::Length;
use uom::si::length::meter;
use uom::si::length::{centimeter, inch, foot};

use template::{move_distance, turn, Direction};

use std::thread;
use std::time::Duration;

fn main() -> Ev3Result<()> {

    // connecting to the robot and constant stuff
    let radius = Length::new::<meter>(0.02667);
	let wheels = (
		LargeMotor::get(MotorPort::OutB)?,
		LargeMotor::get(MotorPort::OutC)?,
	);

    move_distance(
        (&wheels.0, &wheels.1),
        radius,
        Length::new::<foot>(1.5),
    )?;

    turn((&wheels.0, &wheels.1), GyroSensor::find()?, 90, Direction::LEFT)?;

    move_distance(
        (&wheels.0, &wheels.1),
        radius,
        Length::new::<foot>(2.5),
    )?;


	Ok(())
}
