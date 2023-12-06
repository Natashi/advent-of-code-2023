use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap};

struct Card {
	id: u32,
	winnings: HashSet<u32>,
	numbers: Vec<u32>,
}
impl Card {
	fn get_matches(&self) -> u32 {
		self.numbers.iter()
			.filter(|&x| self.winnings.contains(x))
			.count() as u32
	}
	fn get_score(&self) -> u32 {
		let wins = self.get_matches();
		if wins > 0 {
			1 << (wins - 1)
		}
		else { 0u32 }
	}
}

pub fn solve(file: &File) {
	let mut cards = vec![];
	
	for line in BufReader::new(file).lines().flatten() {
		if line.starts_with("Card") {
			let colon_pos = line.chars()
				.position(|x| x == ':')
				.unwrap();
			let divide_pos = line.chars()
				.position(|x| x == '|')
				.unwrap();
			
			let card_id = &line[5..colon_pos]
				.trim()
				.parse::<u32>().unwrap();
			
			let winning_nums = &line[(colon_pos + 1)..divide_pos]
				.split_ascii_whitespace()
				.map(|x| x.trim().parse::<u32>().unwrap())
				.collect::<HashSet<_>>();
			let card_nums = &line[(divide_pos + 1)..]
				.split_ascii_whitespace()
				.map(|x| x.trim().parse::<u32>().unwrap())
				.collect::<Vec<_>>();
			
			cards.push(Card {
				id: *card_id,
				winnings: winning_nums.clone(),
				numbers: card_nums.clone(),
			});
		}
	}
	
	let sum_score = cards.iter()
		.map(|card| card.get_score())
		.sum::<u32>();
	println!("{:?}", sum_score);
	
	{
		let mut cards_count = cards.iter()
			.map(|x| (x.id, 1u32))
			.collect::<HashMap<_, _>>();
		for card in &cards {
			let count = *cards_count.get(&card.id).unwrap();
			let matches = card.get_matches();
			
			// Add more copies of subsequent cards
			for i in 0..matches {
				let id = card.id + 1 + i;
				if let Some(x) = cards_count.get_mut(&id) {
					*x += count;
				}
			}
		}
		println!("{:?}", cards_count.values().sum::<u32>());
	}
}
