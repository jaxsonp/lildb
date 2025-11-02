use std::fmt::Debug;

use lildb::query;

/// A trait every parse tree node must implement. `validate()` validates the semantics of the parse tree, and consumes
/// self, producing a value helpful for creating a full query
pub trait ParseTreeNode: Debug {
	type Product;
	fn validate(self) -> Result<Self::Product, String>;
}

#[derive(Debug)]
pub enum ParseTreeQuery {
	DB(ParseTreeFunction),
}
impl ParseTreeNode for ParseTreeQuery {
	type Product = query::Query;
	fn validate(self) -> Result<Self::Product, String> {
		match self {
			Self::DB(f) => {
				if let Some(f) = f.validate()? {
					Ok(query::Query::DB(f))
				} else {
					Err("Query requires a function".to_string())
				}
			}
		}
	}
}

#[derive(Debug)]
pub enum ParseTreeFunction {
	Function {
		ty: query::FunctionType,
		args: Box<ParseTreeFunctionArgs>,
		chained: Box<ParseTreeFunction>,
	},
	NoFunction,
}
impl ParseTreeNode for ParseTreeFunction {
	type Product = Option<query::Function>;
	fn validate(self) -> Result<Self::Product, String> {
		use ParseTreeFunction::*;
		match self {
			Function { ty, args, chained } => Ok(Some(query::Function::new(
				ty,
				args.validate()?,
				chained.validate()?,
			))),
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
