use ev3dev_lang_rust::motors::MediumMotor;
use ev3dev_lang_rust::sensors::{GyroSensor, Sensor};
use ev3dev_lang_rust::{motors::LargeMotor, Ev3Result};
use uom::si::f32::Length;
use uom::si::length::meter;


pub enum Direction {
    LEFT,
    RIGHT,
}

/// moves the robot forward a certain distance
pub fn move_distance(
	wheels: (&LargeMotor, &LargeMotor),
	radius: Length,
	distance: Length,
) -> Ev3Result<()> {
	let (motor_left, motor_right) = wheels;
    let wheel_speed: i32 = 50;
	motor_left.set_duty_cycle_sp(wheel_speed)?;
	motor_right.set_duty_cycle_sp(wheel_speed)?;

	let rotations =
		distance.get::<meter>() / (2.0 * std::f32::consts::PI * radius.get::<meter>());

	let tachys = rotations * motor_left.get_count_per_rot()? as f32;

	motor_left.set_position(0)?;

	println!("running");

	while motor_left.get_position()? < tachys as i32 {
		motor_right.run_direct()?;
		motor_left.run_direct()?;
	}
	motor_left.stop()?;
	motor_right.stop()?;

	Ok(())
}

pub fn turn(wheel: (&LargeMotor, &LargeMotor), gyro: GyroSensor, angle: u16, direction: Direction) -> Ev3Result<()> {

    let sign = match direction {
        Direction::LEFT => -1,
        Direction::RIGHT => 1,
    };

    wheel.0.set_duty_cycle_sp(50 * sign)?;
    wheel.1.set_duty_cycle_sp(-50 * sign)?;
	gyro.set_mode_gyro_ang()?;
	let prevangle = gyro.get_angle()?;
	let overshoot = 6; // tends to go further than the angle so this is a fun hackfix
	while (gyro.get_angle()? - prevangle).abs() < angle as i32 - overshoot {
		wheel.0.run_direct()?;
		wheel.1.run_direct()?;
	}
	wheel.0.stop()?;
	wheel.1.stop()?;

	Ok(())
}
