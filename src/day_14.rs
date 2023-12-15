use std::fs::File;
use std::collections::HashMap;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::util;

#[derive(Debug, Clone)]
struct Grid {
	objs: Vec<String>,
	width: usize,
	height: usize,
}
impl Grid {
	fn get_hash(&self) -> u64 {
		let mut hasher = DefaultHasher::new();
		self.objs.hash(&mut hasher);
		hasher.finish()
	}
	
	fn get(&mut self, x: usize, y: usize) -> char {
		if let Some(line) = self.objs.get(y) {
			if let Some(obj) = line.get(x..x+1) {
				return obj.chars().next().unwrap();
			}
		}
		panic!();
	}
	fn replace_at(&mut self, x: usize, y: usize, ch: char) {
		if let Some(line) = self.objs.get_mut(y) {
			line.replace_range(x..x+1, &ch.to_string());
		}
	}
	
	fn tilt_cycle(&mut self) {
		self.tilt_up();
		self.tilt_left();
		self.tilt_down();
		self.tilt_right();
	}
	fn tilt_up(&mut self) {
		for x in 0..self.width {
			let tilt = Self::tilt_line(self.height, 
				|y| self.get(x, y));
			for (y, &s) in tilt.iter().enumerate() {
				self.replace_at(x, y, s);
			}
		}
	}
	fn tilt_down(&mut self) {
		for x in 0..self.width {
			let tilt = Self::tilt_line(self.height, 
				|y| self.get(x, self.height - y - 1));
			for (y, &s) in tilt.iter().enumerate() {
				self.replace_at(x, self.height - y - 1, s);
			}
		}
	}
	fn tilt_left(&mut self) {
		for y in 0..self.height {
			let tilt = Self::tilt_line(self.width, 
				|x| self.get(x, y));
			for (x, &s) in tilt.iter().enumerate() {
				self.replace_at(x, y, s);
			}
		}
	}
	fn tilt_right(&mut self) {
		for y in 0..self.height {
			let tilt = Self::tilt_line(self.width, 
				|x| self.get(self.width - x - 1, y));
			for (x, &s) in tilt.iter().enumerate() {
				self.replace_at(self.width - x - 1, y, s);
			}
		}
	}
	
	fn tilt_line<I>(count: usize, mut getter: I) -> Vec<char>
		where I: FnMut(usize) -> char
	{
		let mut slots = (0..count)
			.map(|_| '.')
			.collect::<Vec<_>>();
		
		let mut avail_pos = 0;
		
		for i in 0..count {
			match getter(i) {
				'O' => {
					slots[avail_pos] = 'O';
					avail_pos += 1;
				},
				'#' => {
					slots[i] = '#';
					avail_pos = i + 1;
				},
				_ => {},
			}
		}
		
		slots
	}
	
	fn get_load(&self) -> usize {
		self.objs.iter().enumerate()
			.map(|(y, line)| {
				let y_load = self.height - y;
				line.chars()
					.filter(|&c| c == 'O')
					.count() * y_load
			})
			.sum::<usize>()
	}
	
	fn print(&self) {
		for line in &self.objs {
			println!("{}", line);
		}
	}
}

fn read_input(file: &File) -> Grid {
	let lines = util::read_file_lines(file);
	
	let width = lines[0].len();
	let height = lines.len();
	
	Grid {
		objs: lines,
		width,
		height,
	}
}

fn try_find_repeat(grid: &mut Grid, count: usize) -> Option<(usize, u64)> {
	let mut history: HashMap<u64, usize> = HashMap::new();
	
	for i in 0..count {
		grid.tilt_cycle();
		{
			let hash = grid.get_hash();
			if history.contains_key(&hash) {
				return Some((i, hash));
			}
			history.insert(hash, i);
		}
	}
	None
}
pub fn solve(file: &File) {
	let mut grid = read_input(file);
	
	/* {
		grid.tilt_cycle();
		grid.print();
		println!("{}", grid.get_load());
	} */
	
	{
		let count = 1_000_000_000;
		
		if let Some((rs, rhash)) = try_find_repeat(&mut grid, count) {
			println!("Found repeat at i={} hash=[{}]", rs, grid.get_hash());
			
			let mut cycle_values = vec![];
			
			let mut cycle = 0;
			loop {
				grid.tilt_cycle();
				cycle += 1;
				
				let hash = grid.get_hash();
				cycle_values.push((hash, grid.get_load()));
				
				if grid.get_hash() == rhash {
					println!("Cycle length = {}", cycle);
					
					println!("{:?}", cycle_values);
					grid.print();
					
					// Get value at count with modulo
					{
						// -2 for the 2 extra tilt_cycle at the start of the loop and in try_find_repeat
						let index = (count - rs - 2) % cycle;
						println!("Load = {:?}", cycle_values[index]);
					}
					
					break;
				}
			}
		}
	}
}
