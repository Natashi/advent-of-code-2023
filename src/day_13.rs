use std::fs::File;

use crate::util;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mirror {
	V(usize),
	H(usize),
}
impl Mirror {
	fn get_value(&self) -> usize {
		match self {
			Self::V(x) => x + 1,
			Self::H(x) => (x + 1) * 100,
		}
	}
}

#[derive(Debug)]
struct Terrain {
	map: Vec<String>,
	width: usize,
	height: usize,
}
impl Terrain {
	fn new(lines: &[String]) -> Self {
		Self {
			map: lines.to_vec(),
			width: lines[0].len(),
			height: lines.len(),
		}
	}
	
	fn transpose(&self) -> Self {
		let map_t = (0..self.width)
			.map(|x| {
				self.map.iter()
					.flat_map(|y| y.chars().nth(x))
					.collect::<String>()
			})
			.collect::<Vec<_>>();
		Self {
			map: map_t,
			width: self.height,
			height: self.width,
		}
	}
	
	fn find_mirror(&self, try_smudge: bool) -> Vec<Mirror> {
		fn line_compare(s1: &str, s2: &str) -> usize {
			s1.chars().zip(s2.chars())
				.filter(|&(c1, c2)| c1 != c2)
				.count()
		}
		fn scan_valid(data: &[String], edge: (usize, usize), max_diff: usize) -> bool {
			let mut curr_diff = 0;
			
			for (i, j) in (0..=(edge.0)).rev().zip((edge.1)..data.len()) {
				let line_diff = line_compare(&data[i], &data[j]);
				
				curr_diff += line_diff;
				if curr_diff > max_diff {
					return false;
				}
			}
			
			true
		}
		
		let max_diff = if try_smudge { 1 } else { 0 };
		
		let mut res = vec![];
		
		{
			for i in 0..(self.height - 1) {
				if scan_valid(&self.map, (i, i + 1), max_diff) {
					res.push(Mirror::H(i));
				}
			}
		};
		
		{
			let transpose = self.transpose();
			for i in 0..(transpose.height - 1) {
				if scan_valid(&transpose.map, (i, i + 1), max_diff) {
					res.push(Mirror::V(i));
				}
			}
		};
		
		//println!("{:?}", res);
		res
	}
}

pub fn solve(file: &File) {
	let lines = util::read_file_lines(file);
	
	let terrains = lines
		.split(|x| x.is_empty())
		.map(|x| Terrain::new(x))
		.collect::<Vec<_>>();
	
	{
		let mirrors = terrains.iter()
			.flat_map(|x| x.find_mirror(false))
			.collect::<Vec<_>>();
		
		let mirrors_smudge = {
			let mirrors_new = terrains.iter()
				.map(|x| x.find_mirror(true))
				.collect::<Vec<_>>();
			mirrors_new.iter().zip(mirrors.iter())
				.map(|(nm, om)| {
					// Try to find new mirrors first
					
					match nm.iter().find(|&x| x != om) {
						Some(m) => m,
						None => om,
					}.clone()
				})
				.collect::<Vec<_>>()
		};
		
		println!("{:?}\n", mirrors);
		println!("{:?}\n", mirrors_smudge);
		
		println!("{}", mirrors.iter()
			.map(|m| m.get_value())
			.sum::<usize>());
		println!("{}", mirrors_smudge.iter()
			.map(|m| m.get_value())
			.sum::<usize>());
	}
}
