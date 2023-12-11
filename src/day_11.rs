use std::fs::File;

use itertools::Itertools;

use crate::util;

#[derive(Debug)]
enum Expansion {
	X(usize),
	Y(usize),
}
#[derive(Debug)]
struct Galaxy {
	x: usize,
	y: usize,
}

#[derive(Debug)]
struct Universe {
	width: usize,
	height: usize,
	galaxy_map: Vec<Galaxy>,
	expansions: Vec<Expansion>,
}
impl Universe {
	fn new(file: &File) -> Self {
		let lines = util::read_file_lines(file);
		
		let height = lines.len();
		let width = lines[0].len();
		
		let galaxy_map = lines.iter().enumerate()
			.map(|(y, line)| line.chars().enumerate()
				.filter(|(_, c)| *c == '#')
				.map(move |(x, _)| Galaxy { x, y }))
			.flatten()
			.collect::<Vec<_>>();
		
		let expansions = {
			let expand_x = (0..width)
				.filter(|x| !galaxy_map.iter()
					.map(|g| g.x)
					.contains(x))
				.map(|x| Expansion::X(x));
			let expand_y = (0..height)
				.filter(|y| !galaxy_map.iter()
					.map(|g| g.y)
					.contains(y))
				.map(|y| Expansion::Y(y));
			expand_x.chain(expand_y).collect::<Vec<_>>()
		};
		
		Self {
			width,
			height,
			galaxy_map,
			expansions,
		}
	}
	
	fn galaxy_shortest_dist(&self, galaxy1: &Galaxy, galaxy2: &Galaxy, expansion_factor: usize) -> usize {
		fn _contains(v: usize, x: usize, y: usize) -> bool {
			if x < y	{ v > x && v < y }
			else		{ v > y && v < x }
		}
		
		assert!(expansion_factor > 0);
		
		let mut res = galaxy1.y.abs_diff(galaxy2.y) + galaxy1.x.abs_diff(galaxy2.x);
		{
			let expands = self.expansions.iter()
				.filter(|&e| match e {
					Expansion::X(x) => _contains(*x, galaxy1.x, galaxy2.x),
					Expansion::Y(y) => _contains(*y, galaxy1.y, galaxy2.y),
				})
				.count();
			res += expands * (expansion_factor - 1);
		}
		res
	}
}

pub fn solve(file: &File) {
	let universe = Universe::new(file);
	
	{
		let calc_dists = |expansion_factor: usize| {
			(0..universe.galaxy_map.len()).combinations(2)
				.map(|n| {
					let (n1, n2) = (n[0], n[1]);
					let g1 = &universe.galaxy_map[n1];
					let g2 = &universe.galaxy_map[n2];
					let dist = universe.galaxy_shortest_dist(
						g1, g2, expansion_factor);
					(n1, n2, dist)
				})
				.collect::<Vec<_>>()
		};
		
		{
			let dists = calc_dists(2);
			
			//println!("{:?}", dists);
			println!("{}", dists.iter()
				.map(|(_, _, dist)| dist)
				.sum::<usize>());
		}
		{
			let dists = calc_dists(1_000_000);
			
			//println!("{:?}", dists);
			println!("{}", dists.iter()
				.map(|(_, _, dist)| dist)
				.sum::<usize>());
		}
	}
}
