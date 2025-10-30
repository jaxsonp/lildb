mod lexer;
mod parser;
mod tree;

use lexer::Tokens;

use crate::parser::try_parse_query;

pub fn parse(input: String) -> Option<()> {
	let mut tokens = Tokens::new(input.chars());
	let parsed = try_parse_query(&mut tokens);
	println!("parsed: {:?}", parsed);
	None
}

#[derive(Debug)]
enum FunctionType {
	Table,
	Read,
}

fn main() {
	parse("db.table(abc, xyz).table(123);".to_string());
}
