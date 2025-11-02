use super::*;

#[test]
#[rustfmt::skip]
fn success1() {
	let mut t = Tokens::new("hello world".chars());
	
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("hello".to_string()), loc: SourceLocation::new(0, 0..5) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("world".to_string()), loc: SourceLocation::new(0, 6..11) }));
	assert_eq!(t.next(), None);
}

#[test]
#[rustfmt::skip]
fn success2() {
	let mut t = Tokens::new("db.table(\"Users\").read();\n".chars());

	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("db".to_string()), loc: SourceLocation::new(0, 0..2) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Period, loc: SourceLocation::new(0, 2..3) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("table".to_string()), loc: SourceLocation::new(0, 3..8) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::OpenParen, loc: SourceLocation::new(0, 8..9) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("\"Users\"".to_string()), loc: SourceLocation::new(0, 9..16) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::CloseParen, loc: SourceLocation::new(0, 16..17) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Period, loc: SourceLocation::new(0, 17..18) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("read".to_string()), loc: SourceLocation::new(0, 18..22) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::OpenParen, loc: SourceLocation::new(0, 22..23) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::CloseParen, loc: SourceLocation::new(0, 23..24) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Semicolon, loc: SourceLocation::new(0, 24..25) }));
	assert_eq!(t.next(), None);
}

#[test]
#[rustfmt::skip]
fn success3() {
	let mut t = Tokens::new("abc\n    .efg(\"arg1\", 123)\n    .hij()\n".chars());

	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("abc".to_string()), loc: SourceLocation::new(0, 0..3) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Period, loc: SourceLocation::new(1, 4..5) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("efg".to_string()), loc: SourceLocation::new(1, 5..8) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::OpenParen, loc: SourceLocation::new(1, 8..9) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("\"arg1\"".to_string()), loc: SourceLocation::new(1, 9..15) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Comma, loc: SourceLocation::new(1, 15..16) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("123".to_string()), loc: SourceLocation::new(1, 17..20) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::CloseParen, loc: SourceLocation::new(1, 20..21) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Period, loc: SourceLocation::new(2, 4..5) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::Word("hij".to_string()), loc: SourceLocation::new(2, 5..8) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::OpenParen, loc: SourceLocation::new(2, 8..9) }));
	assert_eq!(t.next(), Some(Token { ty: TokenType::CloseParen, loc: SourceLocation::new(2, 9..10) }));
	assert_eq!(t.next(), None);
}

#[test]
#[rustfmt::skip]
fn whitespace() {
	let mut t = Tokens::new("\t\t  \n          \n\n  \t  ".chars());

	assert_eq!(t.next(), None);
}

#[test]
fn empty() {
	let mut t = Tokens::new("".chars());
	assert_eq!(t.next(), None);
}
