use std::fs::File;
use std::io::{BufReader, BufRead};

struct Range {
	start: u64,
	range: u64,
	dest: u64,
}
struct RangeMap {
	ranges: Vec<Range>,
}
impl RangeMap {
	fn new() -> Self {
		Self {
			ranges: vec![],
		}
	}
	fn map_dest(&self, src: u64) -> u64 {
		for r in &self.ranges {
			if src >= r.start && src < r.start + r.range {
				return (src - r.start) + r.dest;
			}
		}
		src
	}
	fn map_src(&self, dest: u64) -> u64 {
		for r in &self.ranges {
			if dest >= r.dest && dest < r.dest + r.range {
				return (dest - r.dest) + r.start;
			}
		}
		dest
	}
}

pub fn solve(file: &File) {
	let mut seeds = vec![];
	let mut map_seed_soil = RangeMap::new();
	let mut map_soil_fert = RangeMap::new();
	let mut map_fert_water = RangeMap::new();
	let mut map_water_light = RangeMap::new();
	let mut map_light_temp = RangeMap::new();
	let mut map_temp_humid = RangeMap::new();
	let mut map_humid_loc = RangeMap::new();
	
	{
		let mut curr_map: &mut RangeMap = &mut map_seed_soil;
		
		for line in BufReader::new(file).lines().flatten() {
			if let Some(s_line) = line.strip_prefix("seeds: ") {
				seeds = s_line
					.split_ascii_whitespace()
					.map(|x| x.trim().parse::<u64>().unwrap())
					.collect::<Vec<_>>();
			}
			else if let Some(first) = line.chars().next() {
				if !first.is_ascii_digit() {
					let initial = line.chars().take(2).collect::<String>();
					curr_map = match initial.as_str() {
						"se" => &mut map_seed_soil,
						"so" => &mut map_soil_fert,
						"fe" => &mut map_fert_water,
						"wa" => &mut map_water_light,
						"li" => &mut map_light_temp,
						"te" => &mut map_temp_humid,
						"hu" => &mut map_humid_loc,
						_ => panic!(),
					};
				}
				else {
					let nums = line
						.split_ascii_whitespace()
						.map(|x| x.trim().parse::<u64>().unwrap())
						.collect::<Vec<_>>();
					if nums.len() == 3 {
						curr_map.ranges.push(Range {
							dest: nums[0],
							start: nums[1],
							range: nums[2],
						});
					}
				}
			}
		}
	}
	
	{
		let seed_to_location = |seed: u64| {
			// Horrid
			map_humid_loc.map_dest(
				map_temp_humid.map_dest(
					map_light_temp.map_dest(
						map_water_light.map_dest(
							map_fert_water.map_dest(
								map_soil_fert.map_dest(
									map_seed_soil.map_dest(seed)))))))
		};
		let location_to_seed = |location: u64| {
			// Still horrid
			map_seed_soil.map_src(
				map_soil_fert.map_src(
					map_fert_water.map_src(
						map_water_light.map_src(
							map_light_temp.map_src(
								map_temp_humid.map_src(
									map_humid_loc.map_src(location)))))))
		};
		
		{
			let locations = seeds.iter()
				.map(|&seed| seed_to_location(seed))
				.collect::<Vec<_>>();
			println!("{:?}", locations.iter().min().unwrap());
		}
		
		{
			let seed_ranges = seeds.as_slice()
				.chunks(2)
				.map(|x| (x[0], x[1]))
				.collect::<Vec<_>>();
			
			/* let mut min_all = u64::MAX;
			for x in seeds.as_slice().chunks(2) {
				let mut min_now = u64::MAX;
				for i in x[0]..(x[0] + x[1]) {
					min_now = min_now.min(lookup_location(i));
				}
				println!("Range min: {}", min_now);
				min_all = min_all.min(min_now);
			}
			println!("{:?}", min_all); */
			
			let mut first_loc = u64::MAX;
			for i in 0..u64::MAX {
				if i % 10_000_000 == 0 {
					println!("Checking {}", i);
				}
				
				let seed = location_to_seed(i);
				let has_seed = seed_ranges.iter()
					.any(|&(start, count)| seed > start && seed < start + count);
				if has_seed {
					first_loc = i;
					break;
				}
			}
			println!("{:?}", first_loc);
		}
	}
}
