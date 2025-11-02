#[derive(Debug)]
pub enum Query {
	/// Query on db
	DB(Function),
}

#[derive(Debug)]
pub enum FunctionType {
	Table,
	Read,
}
impl FunctionType {}

#[derive(Debug)]
pub struct Function {
	ty: FunctionType,
	args: Vec<Value>,
	chained: Option<Box<Function>>,
}
impl Function {
	pub fn new(ty: FunctionType, args: Vec<Value>, chained: Option<Function>) -> Function {
		Function {
			ty,
			args,
			chained: chained.map(Box::new),
		}
	}
}

#[derive(Debug)]
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
