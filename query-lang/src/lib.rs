mod lexer;
mod parser;

use lexer::Tokens;

use parser::{tree::ParseTreeNode, try_parse_query};

/// Parse an input string
pub fn parse(input: String) -> Result<lildb_api::query::Query, String> {
	let mut tokens = Tokens::new(input.chars());
	let Some(parsed) = try_parse_query(&mut tokens)? else {
		return Err("Input did not contain a query".to_string());
	};
	let q = parsed.validate();
	q
}

fn main() {
	let res = parse("db.table(abc, xyz).read()".to_string());
	println!("result: {:?}", res);
}
