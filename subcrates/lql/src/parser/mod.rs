//!
//! This module contains code implementing a recursive descent parser for LQL. Each `try_parse_*` function will return
//! `Ok(None)` if it decides that the token stream does not contain what is being parsed for (no tokens will be
//! consumed). The functions will return `Err(...)` if it decides that the token stream _does_ contain what is being
//! parsed for, but parsing was unsuccessful after tokens were consumed. `Ok(Some(...))` will be returned parsing was
//! successful.
//!
//! See the readme for grammar definition.
//!
pub mod tree;

use crate::lexer::{Token, TokenType, Tokens};

use tree::*;

/// Describes the output of a parsing function in a recursive descent parser
///
/// The `Result` value represents whether or not the input is invalid, hence a failure to parse, while the successful
/// `Option` value represents whether or not the tokens match this grammar rule
type ParseOutcome<T> = Result<Option<T>, String>;

pub fn try_parse_query(tokens: &mut Tokens) -> ParseOutcome<ParseTreeQuery> {
	let Some(Token {
		ty: TokenType::Word(object),
		..
	}) = tokens.next()
	else {
		return Err(format!(
			"Expected object (line {}, col {})",
			tokens.last_loc.line, tokens.last_loc.start_col
		));
	};

	let function = try_parse_function_call(tokens)?;

	tokens.expect(TokenType::Semicolon)?;

	return Ok(Some(ParseTreeQuery { object, function }));
}

fn try_parse_function_call(tokens: &mut Tokens) -> ParseOutcome<ParseTreeFunctionCall> {
	if let Some(Token {
		ty: TokenType::Period,
		..
	}) = &tokens.peek()
	{
		let _ = tokens.next();
	} else {
		return Ok(Some(ParseTreeFunctionCall::NoFunction));
	}

	let name = if let Some(Token {
		ty: TokenType::Word(_),
		..
	}) = &tokens.peek()
	{
		let TokenType::Word(s) = tokens.next().unwrap().ty else {
			unreachable!();
		};
		s
	} else {
		return Err(format!(
			"Expected function name (line {}, col {})",
			tokens.last_loc.line, tokens.last_loc.start_col
		));
	};

	tokens.expect(TokenType::OpenParen)?;
	let Some(args) = try_parse_function_args(tokens)? else {
		return Err(format!(
			"Expected function arguments (line {}, col {})",
			tokens.last_loc.line, tokens.last_loc.start_col
		));
	};
	tokens.expect(TokenType::CloseParen)?;

	let Some(chained_function) = try_parse_function_call(tokens)? else {
		return Err(format!(
			"Expected function or null (line {}, col {})",
			tokens.last_loc.line, tokens.last_loc.start_col
		));
	};

	Ok(Some(ParseTreeFunctionCall::Function {
		name,
		args: Box::new(args),
		chained: Box::new(chained_function),
	}))
}

fn try_parse_function_args(tokens: &mut Tokens) -> ParseOutcome<ParseTreeFunctionArgs> {
	if tokens.peek().is_some() {
		let Some(value) = try_parse_value(tokens)? else {
			return Ok(Some(ParseTreeFunctionArgs::NoArgs));
		};
		let Some(more_args) = try_parse_more_function_args(tokens)? else {
			return Err(format!(
				"Expected continued argument list or end of arguments (line {}, col {})",
				tokens.last_loc.line, tokens.last_loc.start_col
			));
		};

		return Ok(Some(ParseTreeFunctionArgs::Args {
			value,
			more: Box::new(more_args),
		}));
	} else {
		// e = no args
		return Ok(Some(ParseTreeFunctionArgs::NoArgs));
	}
}

fn try_parse_more_function_args(tokens: &mut Tokens) -> ParseOutcome<ParseTreeMoreFunctionArgs> {
	if let Some(Token {
		ty: TokenType::Comma,
		..
	}) = tokens.peek()
	{
		tokens.next();
		let Some(value) = try_parse_value(tokens)? else {
			return Err(format!(
				"Expected a value (line {}, col {})",
				tokens.last_loc.line, tokens.last_loc.start_col
			));
		};
		let Some(more_args) = try_parse_more_function_args(tokens)? else {
			return Err(format!(
				"Expected continued argument list or end of arguments (line {}, col {})",
				tokens.last_loc.line, tokens.last_loc.start_col
			));
		};

		return Ok(Some(ParseTreeMoreFunctionArgs::MoreArgs {
			value,
			more: Box::new(more_args),
		}));
	} else {
		// e = no args
		return Ok(Some(ParseTreeMoreFunctionArgs::NoMoreArgs));
	}
}

fn try_parse_value(tokens: &mut Tokens) -> ParseOutcome<ParseTreeValue> {
	if let Some(Token {
		ty: TokenType::Word(_),
		..
	}) = tokens.peek()
	{
		let TokenType::Word(s) = tokens.next().unwrap().ty else {
			unreachable!();
		};
		Ok(Some(ParseTreeValue::String(s)))
	} else {
		Ok(None)
	}
}
