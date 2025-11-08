use lildb::query::{self, FunctionCall, Query, functions};
use lql::parse;

#[test]
fn test_empty() {
	let input = "";
	assert!(parse(input.to_string()).is_err())
}

#[test]
fn no_function() {
	let input = "Users;";
	let parsed = parse(input.to_string()).unwrap();
	assert_eq!(parsed, query::Query::new("Users", None));
}

#[test]
fn simple_function() {
	let input = "Users.create();";
	let parsed = parse(input.to_string()).unwrap();
	assert_eq!(
		parsed,
		Query::new(
			"Users",
			Some(FunctionCall::new(
				&functions::createFunction,
				Vec::new(),
				None
			))
		)
	);
}

#[test]
fn unregonized_function() {
	let input = "Users.DOESNOTEXIST();";
	assert!(parse(input.to_string()).is_err());
}

#[test]
fn chained_functions() {
	let input = "Users.ensure_exists().delete();";
	let parsed = parse(input.to_string()).unwrap();
	assert_eq!(
		parsed,
		Query::new(
			"Users",
			Some(FunctionCall::new(
				&functions::ensureExistsFunction,
				Vec::new(),
				Some(FunctionCall::new(
					&functions::deleteFunction,
					Vec::new(),
					None
				))
			))
		)
	);
}
