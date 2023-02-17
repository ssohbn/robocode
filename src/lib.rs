use ev3dev_lang_rust::sensors::{GyroSensor, UltrasonicSensor};
use ev3dev_lang_rust::{motors::LargeMotor, Ev3Result};
use uom::si::f32::Length;
use uom::si::length::{centimeter, meter};

const WHEEL_SPEED: i32 = 40;

pub enum Direction {
    LEFT,
    RIGHT,
}

pub fn move_distance(
	wheels: (&LargeMotor, &LargeMotor),
	radius: Length,
	distance: Length,
) -> Ev3Result<()> {
	let (motor_left, motor_right) = wheels;
	motor_left.set_duty_cycle_sp(WHEEL_SPEED)?;
	motor_right.set_duty_cycle_sp(WHEEL_SPEED)?;

	let rotations =
		distance.get::<meter>() / (2.0 * std::f32::consts::PI * radius.get::<meter>());

	let tachys = rotations * motor_left.get_count_per_rot()? as f32;

	motor_left.set_position(0)?;
	motor_right.set_position(0)?;

    motor_right.run_direct()?;
    motor_left.run_direct()?;

	loop {
		if motor_left.get_position()? >= tachys as i32 || motor_right.get_position()? >= tachys as i32 {
			motor_left.stop()?;
			motor_right.stop()?;
			break;
		}
	}

	Ok(())
}

pub fn turn(wheels: (&LargeMotor, &LargeMotor), gyro: &GyroSensor, angle: u16, direction: Direction) -> Ev3Result<()> {
	let (motor_left, motor_right) = wheels;
	
	let sign = match direction {
        Direction::LEFT => -1,
        Direction::RIGHT => 1,
    };

    motor_left.set_duty_cycle_sp(WHEEL_SPEED * sign)?;
    motor_right.set_duty_cycle_sp(-WHEEL_SPEED * sign)?;

	gyro.set_mode_gyro_ang()?;
	let initial_angle = gyro.get_angle()?;
	let overshoot = 6; // tends to go further than the angle so this is a fun hackfix

	motor_left.run_direct()?;
	motor_right.run_direct()?;

	while (gyro.get_angle()? - initial_angle).abs() < angle as i32 - overshoot {}
	motor_left.stop()?;
	motor_right.stop()?;

	Ok(())
}

pub fn turn_until<C>(wheels: (&LargeMotor, &LargeMotor),  direction: Direction, condition: C) -> Ev3Result<()>
    where C: Fn() -> bool {

	let (motor_left, motor_right) = wheels;

	let sign = match direction {
        Direction::LEFT => -1,
        Direction::RIGHT => 1,
    };

    motor_left.set_duty_cycle_sp(WHEEL_SPEED * sign)?;
    motor_right.set_duty_cycle_sp(-WHEEL_SPEED * sign)?;

	motor_left.run_direct()?;
	motor_right.run_direct()?;

	while condition() {}
	motor_left.stop()?;
	motor_right.stop()?;

	Ok(())

}

pub fn get_obstacle_distance(
	sensor: &UltrasonicSensor
) -> Ev3Result<Length> {
	Ok(Length::new::<centimeter>(sensor.get_distance_centimeters()? as f32))
}
