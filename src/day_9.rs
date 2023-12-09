use std::fs::File;

use crate::util;

trait Diff {
	fn diff(&self) -> Self;
}
impl Diff for Vec<i32> {
	fn diff(&self) -> Self {
		self.iter().as_slice()
			.windows(2)
			.map(|x| x[1] - x[0])
			.collect::<Vec<_>>()
	}
}

fn sequence_predict(sequence: &Vec<i32>) -> i32 {
	let last = *sequence.last().unwrap();
	let diffs = sequence.diff();
	
	let all_zero = diffs.iter().all(|&x| x == 0);
	if all_zero {
		last
	}
	else {
		last + sequence_predict(&diffs)
	}
}
fn sequence_predict_back(sequence: &Vec<i32>) -> i32 {
	let first = *sequence.first().unwrap();
	let diffs = sequence.diff();
	
	let all_zero = diffs.iter().all(|&x| x == 0);
	if all_zero {
		first
	}
	else {
		first - sequence_predict_back(&diffs)
	}
}

pub fn solve(file: &File) {
	let sequences = util::read_file_lines_to_string(file).iter()
		.map(|x| {
			x.split_ascii_whitespace()
				.map(|x| x.parse::<i32>().unwrap())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	
	{
		let predicts = sequences.iter()
			.map(sequence_predict)
			.collect::<Vec<_>>();
		println!("{:?}", predicts);
		println!("{}", predicts.iter().sum::<i32>());
		
		let predicts_back = sequences.iter()
			.map(sequence_predict_back)
			.collect::<Vec<_>>();
		println!("{:?}", predicts_back);
		println!("{}", predicts_back.iter().sum::<i32>());
	}
}
