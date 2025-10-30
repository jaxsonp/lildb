use lildb_ql::parse;

#[test]
fn test_empty() {
	let input = "";
	assert!(parse(input.to_string()).is_none())
}
