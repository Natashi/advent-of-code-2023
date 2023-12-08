use std::fs::File;
use std::collections::HashMap;

use nom::{
	IResult, Parser,
	branch, combinator, sequence, multi,
	character::complete::*,
	bytes::complete::*,
};
use crate::parse_util;

#[derive(Debug, Clone)]
struct Node {
	start: String,
	left: String,
	right: String,
}
impl Node {
	fn get_last_char(&self) -> char {
		self.start.chars().last().unwrap()
	}
}

#[derive(Debug)]
struct Directions {
	cycle: String,
	first: String,
	nodes: HashMap<String, Node>,
}
impl Directions {
	fn _calculate_steps_to<'a, F>(&'a self, start: &'a Node, pred: F) -> (usize, &Node)
		where F: Fn(&Node) -> bool
	{
		let mut cycle = self.cycle.chars().cycle();
		
		let mut step = 0;
		let mut node_now = start;
		
		while !pred(node_now) {
			let walk = cycle.next().unwrap();
			let next = match walk {
				'L'	=> self.nodes.get(&node_now.left),
				_	=> self.nodes.get(&node_now.right),
			}.unwrap();
			
			step += 1;
			node_now = next;
		}
		
		(step, node_now)
	}
	
	fn calculate_steps_to(&self, dest: &str) -> usize {
		let node_start = self.nodes.get(&self.first).unwrap();
		let node_dest = self.nodes.get(dest).unwrap();
		
		let (steps, _) = self._calculate_steps_to(node_start, 
			|n| std::ptr::eq(n, node_dest));
		
		steps
	}
	
	fn calculate_steps_to_z(&self) -> usize {
		let mut nodes_start = self.nodes.iter()
			.filter(|(_, n)| n.get_last_char() == 'A')
			.map(|(_, n)| (0usize, n))
			.collect::<Vec<_>>();
		
		for (sz, node) in &mut nodes_start {
			let (steps, _) = self._calculate_steps_to(node, 
				|n| n.get_last_char() == 'Z');
			*sz = steps;
		}
		
		nodes_start.iter()
			.fold(1, |a, (x, _)| num::integer::lcm(a, *x))
	}
}

fn read_input(file: &File) -> Directions {
	fn take_rl(s: &str) -> IResult<&str, &str> {
		let (s, direction) = take_while1(
			|c| c == 'R' || c == 'L')(s)?;
		
		Ok((s, direction))
	}
	fn take_node(s: &str) -> IResult<&str, Node> {
		let (s, (start, _)) = sequence::pair(
			alphanumeric1, 
			tag(" = ")
		)(s)?;
		let (s, (left, right)) = parse_util::paren(
			sequence::separated_pair(
				alphanumeric1, 
				tag(", "),
				alphanumeric1)
		).parse(s)?;
		
		Ok((s, Node { 
			start: start.to_string(), 
			left: left.to_string(), 
			right: right.to_string(),
		}))
	}
	fn take_file(s: &str) -> IResult<&str, Directions> {
		let (s, cycle) = sequence::terminated(
			take_rl, multispace0)(s)?;
		let (s, nodes) = parse_util::lines(
			take_node).parse(s)?;
		
		Ok((s, Directions { 
			cycle: cycle.to_string(),
			//first: nodes[0].start.clone(),
			first: "AAA".to_string(),
			nodes: nodes.iter()
				.map(|x| (x.start.clone(), x.clone()))
				.collect::<HashMap<_, _>>(),
		}))
	}
	
	let source = parse_util::read_file_to_string(file);
	take_file(&source).unwrap().1
}

pub fn solve(file: &File) {
	let directions = read_input(file);
	
	println!("{:?}", directions);
	
	{
		let steps = directions.calculate_steps_to("ZZZ");
		println!("{}", steps);
		
		let steps_p2 = directions.calculate_steps_to_z();
		println!("{}", steps_p2);
	}
}
