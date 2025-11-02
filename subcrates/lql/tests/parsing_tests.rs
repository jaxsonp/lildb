use lql::parse;

#[test]
fn test_empty() {
	let input = "";
	assert!(parse(input.to_string()).is_err())
}
