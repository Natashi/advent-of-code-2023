use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Number {
	row: usize, col: usize,
	str: String,
	val: u32,
}
#[derive(Debug)]
struct Symbol {
	row: usize, col: usize,
	sym: char,
}
impl Symbol {
	pub fn is_neighbor(&self, num: &Number) -> bool {
		((self.row as i64 - num.row as i64).abs() <= 1)
			&& ((self.col + 1 >= num.col) 
				&& (self.col <= num.col + num.str.len()))
	}
}

fn extract_nums(lines: &[String]) -> Vec<Number> {
	let mut list_nums: Vec<Number> = vec![];
	
	for (i, line) in lines.iter().enumerate() {
		let mut j = 0usize;
		let mut num = String::new();
		
		macro_rules! add_str {
			() => {
				if !num.is_empty() {
					list_nums.push(Number {
						row: i, col: j - num.len(),
						str: num.clone(),
						val: num.parse::<u32>().unwrap(),
					});
					num.clear();
				}
			};
		}
		
		for ch in line.chars() {
			if ch.is_ascii_digit() {
				num.push(ch);
			}
			else {
				add_str!();
				num.clear();
			}
			j += 1;
		}
		add_str!();
	}
	
	list_nums
}
fn extract_symbols(lines: &[String]) -> Vec<Symbol> {
	let mut list_syms: Vec<Symbol> = vec![];
	
	for (i, line) in lines.iter().enumerate() {
		for (j, ch) in line.chars().enumerate() {
			if !ch.is_ascii_digit() && ch != '.' {
				list_syms.push(Symbol {
					row: i, col: j,
					sym: ch,
				});
			}
		}
	}
	
	list_syms
}

pub fn solve(file: &File) {
	let lines = BufReader::new(file)
		.lines().flatten()
		.collect::<Vec<_>>();
	
	let numbers = extract_nums(&lines);
	let symbols = extract_symbols(&lines);
	
	let valid_parts_sum = numbers.iter()
		.filter(|&num| {
			symbols.iter()
				.any(|sym| sym.is_neighbor(num))
		})
		.map(|num| num.val)
		.sum::<u32>();
	
	println!("{:?}", valid_parts_sum);
	
	let gears = symbols.iter()
		.filter(|sym| sym.sym == '*')
		.collect::<Vec<_>>();
	let gear_ratio_sum = gears.iter()
		.map(|&sym_ast| {
			numbers.iter()
				.filter(|&num| sym_ast.is_neighbor(num))
				.map(|num| num.val)
				.collect::<Vec<_>>()
		})
		.filter(|x| x.len() == 2)
		.map(|parts| parts.iter().product::<u32>())
		.sum::<u32>();
	
	println!("{}", gear_ratio_sum);
}
