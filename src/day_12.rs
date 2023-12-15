use std::fs::File;
use std::collections::HashMap;

use itertools::Itertools;

use crate::util;

#[derive(Debug)]
struct Record {
	springs: String,
	groups: Vec<usize>,
}
impl Record {
	fn count_arrangements(&self) -> usize {
		type CKey = (usize, usize);
		type Cache<'a> = HashMap<CKey, usize>;
		let mut cache = Cache::new();
		
		fn _count_arrangements(ss: &str, pos_list: &[usize], c_key: CKey, cache: &mut Cache) -> usize {
			// Cache positions to eliminate repeated calculations, makes a huge difference in part 2
			if let Some(v) = cache.get(&c_key) {
				return *v;
			}
			
			let s_now = if c_key.0 < ss.len() { &ss[c_key.0..] } else { "" };
			let p_now = &pos_list[c_key.1..];
			
			let count = {
				if p_now.is_empty() {
					// No more groups, check if we still have any more "#"
					//    Arrangement is invalid is true
					
					if s_now.chars().any(|c| c == '#') {
						0
					}
					else {
						1
					}
				}
				else if let Some(symbol) = s_now.chars().next() {
					// Still needs more groups (input must not be empty already)
					
					let mut _count = 0;
					
					if symbol == '.' || symbol == '?' {
						// Spring is operational, just skip
						
						_count += _count_arrangements(ss, pos_list, 
							(c_key.0 + 1, c_key.1), cache);
					}
					if symbol == '#' || symbol == '?' {
						// See if there are enough [#?] for the group
						
						let expected = p_now[0];
						
						let count_ok = !s_now.chars()
							.take(expected)
							.any(|c| c == '.')
							&& (s_now.len() >= expected);
						
						if count_ok {
							if s_now.len() == expected {
								// No more input left after this, must also be the last group
								
								_count += if p_now.len() == 1 {
									1
								} else { 0 };
							}
							else {
								// More input after, next character must not be a #
								
								match s_now.chars().nth(expected) {
									Some('.') | Some('?') => {
										// Move on to the next group
										
										let next = expected + 1;
										_count += _count_arrangements(ss, pos_list, 
											(c_key.0 + next, c_key.1 + 1), cache) + 1;
									},
									_ => {},
								}
							}
						}
					}
					
					_count
				}
				else { 0 }
			};
			
			if false {
				let pre = (0..c_key.0).map(|_| ' ').collect::<String>();
				println!("{}{} {:?} -> {}", pre, s_now, p_now, count);
			}
			
			cache.insert(c_key, count);
			count
		}
		
		_count_arrangements(&self.springs, &self.groups, 
			(0, 0), &mut cache)
	}
}

fn read_input(lines: &[String], part2: bool) -> Vec<Record> {
	let records = lines.iter()
		.map(|line| {
			let mut s_line = line.split_ascii_whitespace();
			let mut springs = s_line.next().unwrap().to_string();
			let mut groups = s_line.next().unwrap()
				.split(',')
				.map(|n| n.parse::<usize>().unwrap())
				.collect::<Vec<_>>();
			
			if part2 {
				springs = (0..5)
					.map(|_| &springs)
					.join("?");
				
				let mut groups_p2 = vec![];
				(0..5).for_each(|_| groups_p2.extend_from_slice(&groups));
				groups = groups_p2;
			}
			//springs.push('.');	// Add terminator
			
			Record {
				springs, groups,
			}
		})
		.collect::<Vec<_>>();
	records
}

pub fn solve(file: &File) {
	let lines = util::read_file_lines(file);
	
	for p in [false, true] {
		let records = read_input(&lines, p);
		
		let count_possible = records.iter()
			.map(|r| {
				r.count_arrangements()
			})
			.collect::<Vec<_>>();
		println!("{:?}", count_possible);
		println!("{}", count_possible.iter().sum::<usize>());
	}
}
