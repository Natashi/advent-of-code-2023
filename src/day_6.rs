use std::fs::File;
use std::io::{BufReader, BufRead};

struct Race {
	time: u64,
	record: u64,
}
impl Race {
	fn strategy_viable(&self, acc_time: u64) -> bool {
		acc_time * (self.time - acc_time) > self.record
	}
	fn get_viable_strategy_count(&self) -> usize {
		(1..self.time)
			.filter(|&x| self.strategy_viable(x))
			.count()
	}
}

fn read_input(file: &File) -> Vec<Race> {
	let mut times = vec![];
	let mut dists = vec![];
	
	for line in BufReader::new(file).lines().flatten() {
		if let Some(s_line) = line.strip_prefix("Time: ") {
			times = s_line
				.split_ascii_whitespace()
				.map(|x| x.trim().parse::<u64>().unwrap())
				.collect::<Vec<_>>();
		}
		else if let Some(s_line) = line.strip_prefix("Distance: ") {
			dists = s_line
				.split_ascii_whitespace()
				.map(|x| x.trim().parse::<u64>().unwrap())
				.collect::<Vec<_>>();
		}
	}
	
	times.iter().zip(dists.iter())
		.map(|(t, d)| Race {
			time: *t,
			record: *d,
		})
		.collect::<Vec<_>>()
}

pub fn solve(file: &File) {
	let races = read_input(file);
	
	{
		let strat_counts = races.iter()
			.map(|x| x.get_viable_strategy_count())
			.collect::<Vec<_>>();
		
		println!("{:?}", strat_counts);
		println!("{:?}", strat_counts.iter().product::<usize>());
	}
	
	{
		let times_cat = races.iter()
			.map(|x| x.time.to_string())
			.collect::<Vec<_>>().join("")
			.parse::<u64>().unwrap();
		let dists_cat = races.iter()
			.map(|x| x.record.to_string())
			.collect::<Vec<_>>().join("")
			.parse::<u64>().unwrap();
		
		let race_p2 = Race {
			time: times_cat,
			record: dists_cat,
		};
		println!("{:?}", race_p2.get_viable_strategy_count());
	}
}
