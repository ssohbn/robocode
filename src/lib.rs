use ev3dev_lang_rust::sensors::{GyroSensor, UltrasonicSensor};
use ev3dev_lang_rust::{motors::LargeMotor, Ev3Result, Ev3Error};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use uom::si::f32::Length;
use uom::si::length::{centimeter, meter};

const DEFAULT_SPEED: i32 = 40;

pub enum Direction{
	LEFT,
	RIGHT,
}

impl Direction{
	fn sign(&self) -> i32 {
		match self {
			Direction::LEFT => -1,
			Direction::RIGHT => 1,
		}
	}
}

pub struct Wheels{
	pub motors: (LargeMotor, LargeMotor),
	pub radius: Length,
}

pub struct Bot{
	pub wheels: Wheels,
	pub gyro: GyroSensor,
	pub ultrasonic: Option<UltrasonicSensor>,
	pub running: Arc<AtomicBool>,
}

pub struct MoveOptions{
	pub forward: bool,
	pub speed: i32,
}

impl Default for MoveOptions{
	fn default() -> Self {
		Self {
			forward: true,
			speed: DEFAULT_SPEED,
		}
	}
}

pub struct TurnOptions{
	pub speed: i32,
	pub direction: Direction,
}

impl Default for TurnOptions{
	fn default() -> Self {
		Self {
			speed: DEFAULT_SPEED,
			direction: Direction::LEFT,
		}
	}
}

impl Bot{
	//Moves the robot until the given condition is met.
	pub fn move_until<C>(&self, options: &MoveOptions, until: C) -> Ev3Result<()>
	where C: Fn() -> Ev3Result<bool> {
		let (left, right) = &self.wheels.motors;
		let speed = if options.forward { options.speed } else { -options.speed };
		left.set_duty_cycle_sp(speed)?;
		right.set_duty_cycle_sp(speed)?;

		left.set_position(0)?;
		right.set_position(0)?;

		left.run_direct()?;
		right.run_direct()?;

		loop {
			if until()? || !self.running.load(Ordering::SeqCst) {
				self.stop_movement()?;
				break;
			}
		}
		
		Ok(())
	}

	//Moves the robot by the given distance.
	pub fn move_distance(&self, options: &MoveOptions, distance: Length) -> Ev3Result<()> {
		let (left, right) = &self.wheels.motors;
		self.move_until(
			options,
			|| {
				let rotations =
					distance.get::<meter>() / (2.0 * std::f32::consts::PI * self.wheels.radius.get::<meter>());
				let tachys = rotations * left.get_count_per_rot()? as f32;
				Ok(left.get_position()?.abs() >= tachys as i32 || right.get_position()?.abs() >= tachys as i32)
			},
		)
	}

	//Turns the robot until the given condition is met.
	pub fn turn_until<C>(&self, options: &TurnOptions, until: C) -> Ev3Result<()> 
	where C: Fn() -> Ev3Result<bool> {
		let (left, right) = &self.wheels.motors;
		left.set_duty_cycle_sp(options.speed * options.direction.sign())?;
		right.set_duty_cycle_sp(options.speed * -options.direction.sign())?;

		left.run_direct()?;
		right.run_direct()?;

		loop{
			if until()? || !self.running.load(Ordering::SeqCst) {
				self.stop_movement()?;
				break;
			}
		}

		Ok(())
	}

	//Turns the robot by the given angle in degrees.
	pub fn turn_angle(&self, options: &TurnOptions, angle: i32) -> Ev3Result<()> {
		self.gyro.set_mode_gyro_ang()?;
		let initial_angle = self.gyro.get_angle()?;
		self.turn_until(
			options,
			|| {
				Ok(self.gyro.get_angle()? - initial_angle >= angle)
			},
		)
	}

	//Stops the robot's movements.
	pub fn stop_movement(&self) -> Ev3Result<()> {
		let (left, right) = &self.wheels.motors;
		left.stop()?;
		right.stop()?;
		Ok(())
	}

	//Get the distance from the obstacle in front of the robot.
	pub fn get_distance_from_obstacle(&self) -> Ev3Result<Length> {
		if let Some(ultrasonic) = &self.ultrasonic {
			return Ok(Length::new::<centimeter>(ultrasonic.get_distance_centimeters()?));
		}
		Err(Ev3Error::NotConnected{device: "Ultrasonic Sensor".to_string(), port: None})
	}
}
