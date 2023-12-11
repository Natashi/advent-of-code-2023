use std::fs::File;
use std::io::{BufReader, BufRead, Read};

pub fn read_file_to_string(file: &File) -> String {
	let mut read = BufReader::new(file);
	let mut s = String::new();
	read.read_to_string(&mut s).unwrap();
	s
}
pub fn read_file_lines(file: &File) -> Vec<String> {
	BufReader::new(file).lines()
		.flatten()
		.collect::<Vec<_>>()
}

pub mod parse {
	use nom::{
		IResult,
		Parser,
		error::ParseError,
		
		multi::*,
		sequence::*,
		combinator::*,
		bytes::complete::*,
		character::complete::*,
	};
	
	pub fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
		where
		F: Parser<&'a str, O, E>,
	{
		delimited(
			multispace0,
			inner,
			multispace0
		)
	}

	pub fn paren<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
		where
		F: Parser<&'a str, O, E>,
	{
		delimited(
			tag("("),
			inner,
			tag(")"),
		)
	}

	pub fn lines<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, Vec<O>, E>
		where
		F: Parser<&'a str, O, E>,
	{
		many1(
			terminated(
				inner, 
				opt(line_ending))
		)
	}
}
