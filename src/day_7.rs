use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

use std::cmp::Ordering;

fn card_to_score(face: char, enable_joker: bool) -> u32 {
	let map = if !enable_joker { "23456789TJQKA" }
								else { "J23456789TQKA" };
	match map.find(face) {
		Some(p) => p as u32 + 1,
		None => 0,
	}
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
enum HandType {
	HighCard,
	OnePair,
	TwoPair,
	ThreeKind,
	FullHouse,
	FourKind,
	FiveKind,
}

#[derive(Debug)]
struct HandResult {
	hand_type: HandType,
	card_scores: Vec<u32>,
}
#[derive(Debug)]
struct Hand {
	cards: String,
	bet: u32,
	
	map_freq: HashMap<char, u32>,
	hand_result: Option<HandResult>,
}
impl Hand {
	fn new(cards: &str, bet: u32) -> Self {
		let map_freq = cards.chars()
			.fold(HashMap::new(), |mut hm, c| {
				*hm.entry(c).or_insert(0) += 1;
				hm
			});
		Hand {
			cards: cards.to_string(),
			bet,
			map_freq,
			hand_result: None,
		}
	}
	
	fn calculate_hand_result(&mut self, enable_joker: bool) {
		let map_freq_joker = if enable_joker {
			let mut new_map = self.map_freq.clone();
			
			if let Some(&joker_count) = new_map.get(&'J') {
				if joker_count == 0 || joker_count == 5 { None }
				else {
					new_map.remove(&'J');
					
					if let Some((_, v)) = new_map.iter_mut().max_by_key(|(_, v)| **v) {
						*v += joker_count;
					}
					
					Some(new_map)
				}
			}
			else { None }
		}
		else {
			None
		};
		
		let use_map_freq = {
			if enable_joker && map_freq_joker.is_some() {
				map_freq_joker.as_ref().unwrap()
			}
			else { &self.map_freq }
		};
		let hand_type = Self::calculate_hand_type(use_map_freq);
		
		let card_scores = self.cards.chars()
			.map(|c| card_to_score(c, enable_joker))
			.collect::<Vec<_>>();
		
		self.hand_result = Some(HandResult { 
			hand_type,
			card_scores, 
		});
	}
	fn calculate_hand_type(map_freq: &HashMap<char, u32>) -> HandType {
		let (&highest_face, &highest_freq) = map_freq.iter()
			.max_by_key(|&(_, v)| v).unwrap();
		
		let hand_type = match highest_freq {
			5 => HandType::FiveKind,
			4 => HandType::FourKind,
			3 => {
				if map_freq.len() == 2 {
					HandType::FullHouse
				}
				else {
					HandType::ThreeKind
				}
			},
			2 => {
				let other = map_freq.iter()
					.filter(|&(f, c)| *c == 2 && *f != highest_face)
					.nth(0);
				match other {
					Some(_) => HandType::TwoPair,
					None => HandType::OnePair,
				}
			},
			1 => HandType::HighCard,
			_ => panic!(),
		};
		
		hand_type
	}
	
	// Too lazy to implement PartialEq too
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let Some(sr) = &self.hand_result else { panic!(); };
		let Some(or) = &other.hand_result else { panic!(); };
		
		match sr.hand_type.partial_cmp(&or.hand_type) {
			Some(Ordering::Equal) => {}
			ord => return ord,
		};
		for (ss, os) in sr.card_scores.iter().zip(or.card_scores.iter()).take(5) {
			match ss.partial_cmp(os) {
				Some(Ordering::Equal) => {}
				ord => return ord,
			};
		}
		panic!()
	}
}

pub fn solve(file: &File) {
	let mut hands = vec![];
	
	for line in BufReader::new(file).lines().flatten() {
		let s_line = line
			.split_ascii_whitespace()
			.collect::<Vec<_>>();
		if s_line.len() == 2 {
			let cards = s_line[0];
			let bet = s_line[1].parse::<u32>().unwrap();
			hands.push(Hand::new(cards, bet));
		}
	}
	
	{
		for i in &mut hands {
			i.calculate_hand_result(true);
		}
		hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
		
		for i in &hands {
			println!("{:?} {:?}", i.cards, i.hand_result.as_ref().unwrap());
		}
		
		{
			let winning = (0..hands.len()).zip(hands.iter())
				.fold(0, |w, (rank, hand)| {
					w + ((rank + 1) as u32 * hand.bet)
				});
			println!("{}", winning);
		}
	}
	
}
