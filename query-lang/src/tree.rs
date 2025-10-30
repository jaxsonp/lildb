use crate::FunctionType;

#[derive(Debug)]
pub struct ParseTreeQuery {
	pub f: Option<ParseTreeFunction>,
}

#[derive(Debug)]
pub enum ParseTreeFunction {
	Function {
		ty: FunctionType,
		args: Box<ParseTreeFunctionArgs>,
		chained_function: Box<ParseTreeFunction>,
	},
	None,
}

#[derive(Debug)]
pub enum ParseTreeFunctionArgs {
	Args {
		value: Box<ParseTreeValue>,
		more: Box<ParseTreeMoreFunctionArgs>,
	},
	NoArgs,
}

#[derive(Debug)]
pub enum ParseTreeMoreFunctionArgs {
	MoreArgs {
		value: Box<ParseTreeValue>,
		more: Box<ParseTreeMoreFunctionArgs>,
	},
	NoMoreArgs,
}

#[derive(Debug)]
pub enum ParseTreeValue {
	String(String),
}
