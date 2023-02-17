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
        //Keep turning 5 degrees left while there is something <5 cm away. 
        while get_obstacle_distance(&ultrasonic_sensor)? < Length::new::<centimeter>(5.0) {
            turn(wheels, gyro_sensor, 5, Direction::LEFT);
        }
        //Continuously move forward by 1cm while there is nothing in the way.
        move_distance(wheels, Length::new::<meter>(0.02667), Length::new::<centimeter>(1.0));
    }

	Ok(())
}
