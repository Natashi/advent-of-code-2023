use std::fs::File;
use std::io::{BufReader, BufRead};

fn extract_number(s: &str) -> Option<u32> {
	let digits = s.chars()
		.filter(|c| c.is_ascii_digit())
		.map(|x| (x as u32) - ('0' as u32))
		.collect::<Vec<_>>();
	if !digits.is_empty() {
		Some(digits.first().unwrap() * 10 + digits.last().unwrap())
	}
	else { None }
}
fn line_unmangle(s: &str) -> String {
	let mut res = String::from(s);
	
	let patterns = vec![
		// Edge cases (horrid)
		("oneight", "18"),
		("twone", "21"),
		("threeight", "38"),
		("fiveight", "58"),
		("sevenine", "79"),
		("eightwo", "82"),
		("eighthree", "83"),
		("nineight", "98"),
		
		("one", "1"),
		("two", "2"),
		("three", "3"),
		("four", "4"),
		("five", "5"),
		("six", "6"),
		("seven", "7"),
		("eight", "8"),
		("nine", "9"),
	];
	
	let mut i = 0;
	while i < res.len() {
		let ss = &res[i..];
		for &(p, n) in &patterns {
			if ss.starts_with(p) {
				res.replace_range(i..(i + p.len()), n);
				break;
			}
		}
		i += 1;
	}
	
	res
}

pub fn solve(file: &File) {
	let lines = BufReader::new(file).lines()
		.flatten()
		.collect::<Vec<_>>();
	
	{
		let sum = lines.iter()
			.filter_map(|line| extract_number(line))
			.sum::<u32>();
		println!("{}", sum);
	}
	{
		let sum = lines.iter()
			.map(|line| line_unmangle(line))
			.filter_map(|line| extract_number(&line))
			.sum::<u32>();
		println!("{}", sum);
	}
}
