use std::fs::File;
use std::io::{BufReader, BufRead};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn solve(file: &File) {
	let mut possible_games: Vec<u32> = vec![];
	let mut cube_powers: Vec<u32> = vec![];
	
	for line in BufReader::new(file).lines().flatten() {
		if line.starts_with("Game") {
			let split_pos = line.chars()
				.position(|x| x == ':')
				.unwrap();
			
			let game_id = line.chars()
				.skip(5).take(split_pos - 5)
				.collect::<String>()
				.parse::<u32>().unwrap();
			
			let mut subset_max_cubes = (0, 0, 0);
			
			// all is short-circuiting, so do a collect before
			let possible = line.chars()
				.skip(split_pos + 1)
				.collect::<String>()
				.split(';')
				.map(|subset| {
					let maps = subset
						.split(',')
						.map(|x| x.trim())
						.map(|action| {
							let mut itr = action.split_whitespace();
							let count = itr.next().unwrap().parse::<u32>().unwrap();
							let color = itr.next().unwrap();
							
							match color {
								"red" => {
									if count > subset_max_cubes.0 {
										subset_max_cubes.0 = count;
									}
									count <= MAX_RED
								},
								"green" => {
									if count > subset_max_cubes.1 {
										subset_max_cubes.1 = count;
									}
									count <= MAX_GREEN
								},
								"blue" => {
									if count > subset_max_cubes.2 {
										subset_max_cubes.2 = count;
									}
									count <= MAX_BLUE
								},
								_ => panic!(),
							}
						})
						.collect::<Vec<bool>>();
					maps.iter().all(|&x| x)
				})
				.collect::<Vec<bool>>()
				.iter().all(|&x| x);
			
			if possible {
				possible_games.push(game_id);
			}
			
			{
				let power = subset_max_cubes.0
					* subset_max_cubes.1
					* subset_max_cubes.2;
				cube_powers.push(power);
			}
		}
	}
	
	println!("{:?}", possible_games.iter().sum::<u32>());
	println!("{:?}", cube_powers.iter().sum::<u32>());
}
