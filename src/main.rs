use std::env;
use std::process::exit;
use std::fs::File;

use seq_macro::seq;
use paste::paste;

mod util;

seq!(D in 1..=25 {
	paste! {
		mod [<day_ D>];
	}
});

fn main() {
	let argv: Vec<String> = env::args().collect();
	
	if argv.len() < 3 {
		println!("Hello, world!");
		exit(-1);
	}
	
	let day = argv[1].parse::<u32>().unwrap();
	let path = &argv[2];
	
	let file = File::open(path).unwrap();
	
	// Horrid
	macro_rules! day_matches {
		( $( $x:literal ),* ) => {
			{
				paste! {
					match day {
						$( $x => [<day_ $x>]::solve(&file), )*
						_ => unreachable!(),
					}
				}
			}
		};
	}
	day_matches!(
		1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 
		11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
		21, 22, 23, 24, 25
	);
}
