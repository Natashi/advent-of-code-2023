use std::env;
use std::process::exit;
use std::fs::File;

mod parse_util;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
	let argv: Vec<String> = env::args().collect();
	
	if argv.len() < 3 {
		println!("Hello, world!");
		exit(-1);
	}
	
	let day = argv[1].parse::<u32>().unwrap();
	let path = &argv[2];
	
	let file = File::open(path).unwrap();
	
	match day {
		1 => day_1::solve(&file),
		2 => day_2::solve(&file),
		3 => day_3::solve(&file),
		4 => day_4::solve(&file),
		5 => day_5::solve(&file),
		6 => day_6::solve(&file),
		7 => day_7::solve(&file),
		8 => day_8::solve(&file),
		9 => day_9::solve(&file),
		_ => panic!(),
	}
}
