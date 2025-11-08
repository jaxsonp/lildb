pub mod functions;
mod types;

use functions::FunctionDef;

pub use types::Type;

#[derive(Debug, PartialEq, Eq)]
pub struct Query {
	object_name: String,
	function: Option<FunctionCall>,
}
impl Query {
	pub fn new<S: Into<String>>(object_name: S, function: Option<FunctionCall>) -> Query {
		Query {
			object_name: object_name.into(),
			function,
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionCall {
	function: &'static FunctionDef,
	args: Vec<Value>,
	chained: Option<Box<FunctionCall>>,
}
impl FunctionCall {
	pub fn new(
		function: &'static FunctionDef,
		args: Vec<Value>,
		chained: Option<FunctionCall>,
	) -> FunctionCall {
		FunctionCall {
			function,
			args,
			chained: chained.map(Box::new),
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
	String(String),
}

/*
db
	.table("users")
	.where_(gt(col("age"), 18))
	.and(eq(col("active"), true))
	.select(&["name", "email"])
	.order_by("age" desc)
	.limit(10)

tokens:
word period word lparen word rparen

AST:
limit: R
	sort: R
		select: R
			where: R
				read: R
					table: T
						database: D

Parse tree:
function
-name: limit
-args: [10]
-on:
	function
	-name: sort
	-args: "age", desc
	-on:
		table

OR



limit(sort(select(where(read(table(database))))))
R     R    R      R     R    T     D

db
*/
