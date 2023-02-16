use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	/// percentage between -100% and 100% power
	#[arg(short, long, default_value_t = 50)]
	pub motor_speed: i32,
}
