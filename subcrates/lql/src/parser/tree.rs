use std::fmt::Debug;

use lildb::query;

/// A trait every parse tree node must implement. `validate()` validates the semantics of the parse tree, and consumes
/// self, producing a value helpful for creating a full query
pub trait ParseTreeNode: Debug {
	type Product;
	fn validate(self) -> Result<Self::Product, String>;
}

#[derive(Debug)]
pub struct ParseTreeQuery {
	pub object: String,
	pub function: Option<ParseTreeFunctionCall>,
}
impl ParseTreeNode for ParseTreeQuery {
	type Product = query::Query;
	fn validate(self) -> Result<Self::Product, String> {
		let function = match self.function {
			Some(f) => f.validate()?,
			None => None,
		};
		return Ok(query::Query::new(self.object, function));
	}
}

#[derive(Debug)]
pub enum ParseTreeFunctionCall {
	Function {
		name: String,
		args: Box<ParseTreeFunctionArgs>,
		chained: Box<ParseTreeFunctionCall>,
	},
	NoFunction,
}
impl ParseTreeNode for ParseTreeFunctionCall {
	type Product = Option<query::FunctionCall>;
	fn validate(self) -> Result<Self::Product, String> {
		use ParseTreeFunctionCall::*;
		match self {
			Function {
				name,
				args,
				chained,
			} => {
				if let Some(f) = query::functions::find_function(&name) {
					Ok(Some(query::FunctionCall::new(
						f,
						args.validate()?,
						chained.validate()?,
					)))
				} else {
					Err(format!("Unrecognized function: \"{}\"", name))
				}
			}
			NoFunction => Ok(None),
		}
	}
}

#[derive(Debug)]
pub enum ParseTreeFunctionArgs {
	Args {
		value: ParseTreeValue,
		more: Box<ParseTreeMoreFunctionArgs>,
	},
	NoArgs,
}
impl ParseTreeNode for ParseTreeFunctionArgs {
	type Product = Vec<query::Value>;
	fn validate(self) -> Result<Self::Product, String> {
		use ParseTreeFunctionArgs::*;
		match self {
			Args { value, more } => {
				let mut following_args = more.validate()?;
				following_args.push(value.validate()?);
				Ok(following_args)
			}
			NoArgs => Ok(Vec::new()),
		}
	}
}

#[derive(Debug)]
pub enum ParseTreeMoreFunctionArgs {
	MoreArgs {
		value: ParseTreeValue,
		more: Box<ParseTreeMoreFunctionArgs>,
	},
	NoMoreArgs,
}
impl ParseTreeNode for ParseTreeMoreFunctionArgs {
	type Product = Vec<query::Value>;
	fn validate(self) -> Result<Self::Product, String> {
		use ParseTreeMoreFunctionArgs::*;
		match self {
			MoreArgs { value, more } => {
				let mut following_args = more.validate()?;
				following_args.push(value.validate()?);
				Ok(following_args)
			}
			NoMoreArgs => Ok(Vec::new()),
		}
	}
}

#[derive(Debug)]
pub enum ParseTreeValue {
	String(String),
}
impl ParseTreeNode for ParseTreeValue {
	type Product = query::Value;
	fn validate(self) -> Result<Self::Product, String> {
		use ParseTreeValue::*;
		match self {
			String(s) => Ok(query::Value::String(s)),
		}
	}
}
