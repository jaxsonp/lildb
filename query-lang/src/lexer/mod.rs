#[cfg(test)]
mod tests;

use std::{fmt, iter::Peekable, ops::Range, str::Chars};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SourceLocation {
	pub line: u32,
	pub start_col: u32,
	pub end_col: u32,
}
impl SourceLocation {
	pub const fn new(line: u32, cols: Range<u32>) -> SourceLocation {
		SourceLocation {
			line,
			start_col: cols.start,
			end_col: cols.end,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
	pub ty: TokenType,
	pub loc: SourceLocation,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
	Word(String),
	OpenParen,
	CloseParen,
	Period,
	Comma,
	Semicolon,
}
impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TokenType::Word(s) => write!(f, "\"{s}\""),
			TokenType::OpenParen => write!(f, "("),
			TokenType::CloseParen => write!(f, ")"),
			TokenType::Period => write!(f, "."),
			TokenType::Comma => write!(f, ","),
			TokenType::Semicolon => write!(f, ";"),
		}
	}
}

/// Iterator that produces tokens from a `Chars` iterator, is also peekable
pub struct Tokens<'input> {
	chars: Peekable<Chars<'input>>,
	/// Token that was peeked
	peeked: Option<Token>,
	/// Location of last token outputted, for error reporting
	pub last_loc: SourceLocation,
	pub line: u32,
	pub col: u32,
}
impl<'input> Tokens<'input> {
	pub fn new(input: Chars<'input>) -> Tokens<'input> {
		Tokens {
			chars: input.peekable(),
			peeked: None,
			last_loc: SourceLocation::new(0, 0..0),
			line: 0,
			col: 0,
		}
	}

	/// Get a reference to the next token up in the iterator
	pub fn peek(&mut self) -> Option<&Token> {
		if self.peeked.is_none() {
			self.peeked = self.next();
		}
		return self.peeked.as_ref();
	}

	/// Consumes token and errors if it does not have the expected type
	pub fn expect(&mut self, ty: TokenType) -> Result<(), String> {
		if let Some(tok) = self.next() {
			if tok.ty == ty {
				Ok(())
			} else {
				Err(format!(
					"Expected \"{}\", found {} (line {}, col {})",
					ty, tok.ty, tok.loc.line, tok.loc.start_col
				))
			}
		} else {
			Err(format!("Expected \"{}\", found EOF", ty))
		}
	}
}
impl<'input> Iterator for Tokens<'input> {
	type Item = Token;

	fn next(&mut self) -> Option<Token> {
		// take peeked token if it exists
		if let Some(tok) = self.peeked.take() {
			self.last_loc = tok.loc;
			return Some(tok);
		}

		let c = match self.chars.next() {
			Some(c) => c,
			None => return None,
		};
		self.col += 1;

		// skip all whitespace
		if c.is_whitespace() {
			if c == '\n' {
				self.line += 1;
				self.col = 0;
			}
			while let Some(next) = self.chars.peek()
				&& next.is_whitespace()
			{
				if *next == '\n' {
					self.line += 1;
					self.col = 0;
				} else {
					self.col += 1;
				}
				self.chars.next();
			}
			return self.next();
		}

		// single char tokens
		let mut single_char_token = |ty: TokenType| -> Token {
			let t = Token {
				ty,
				loc: SourceLocation::new(self.line, (self.col - 1)..self.col),
			};
			self.last_loc = t.loc;
			t
		};
		match c {
			'(' => return Some(single_char_token(TokenType::OpenParen)),
			')' => return Some(single_char_token(TokenType::CloseParen)),
			'.' => return Some(single_char_token(TokenType::Period)),
			',' => return Some(single_char_token(TokenType::Comma)),
			';' => return Some(single_char_token(TokenType::Semicolon)),
			_ => {}
		}

		// word token
		let start_col = self.col - 1;
		let mut word = String::from(c);
		while let Some(next) = self.chars.peek()
			&& *next != '('
			&& *next != ')'
			&& *next != '.'
			&& *next != ','
			&& *next != ';'
			&& !next.is_whitespace()
		{
			word.push(self.chars.next().unwrap());
			self.col += 1;
		}

		let t = Token {
			ty: TokenType::Word(word),
			loc: SourceLocation::new(self.line, start_col..self.col),
		};
		self.last_loc = t.loc;
		return Some(t);
	}
}
