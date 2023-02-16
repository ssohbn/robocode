extern crate ev3dev_lang_rust;

use ev3dev_lang_rust::motors::{LargeMotor, MotorPort};
use ev3dev_lang_rust::sensors::{GyroSensor, UltrasonicSensor};
use ev3dev_lang_rust::Ev3Result;
use uom::si::f32::Length;
use uom::si::length::{centimeter, meter};
use robocode::{Direction, get_obstacle_distance, move_distance, turn};

fn main() -> Ev3Result<()> {
    let wheels = (&LargeMotor::get(MotorPort::OutB)?, &LargeMotor::get(MotorPort::OutC)?);
    wheels.0.stop()?;
    wheels.1.stop()?;
    let gyro_sensor = &GyroSensor::find()?;
    let ultrasonic_sensor = &UltrasonicSensor::find()?;

    loop {
        println!("Moving forward");
        move_distance(wheels, Length::new::<meter>(0.02667), Length::new::<centimeter>(1.0));
        let distance = get_obstacle_distance(&ultrasonic_sensor)?;
        println!("Distance: {}", distance.get::<centimeter>());
        if distance < Length::new::<centimeter>(5.0) {
            println!("Obstacle detected");
            turn(wheels, gyro_sensor, 180, Direction::LEFT);
        }
    }

	Ok(())
}
