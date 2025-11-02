mod lexer;
mod parser;

use lildb::query;

use lexer::Tokens;
use parser::{tree::ParseTreeNode, try_parse_query};

/// Parse a string into a `Query`
pub fn parse(input: String) -> Result<query::Query, String> {
	let mut tokens = Tokens::new(input.chars());
	let Some(parsed) = try_parse_query(&mut tokens)? else {
		return Err("Input did not contain a query".to_string());
	};
	let q = parsed.validate();
	q
}
