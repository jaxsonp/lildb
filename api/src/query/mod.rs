use std::io::{self, Read};

use crate::codec::{Decodable, Encodable};

#[derive(Debug)]
pub enum Query {
	/// Query on db
	DB(Function),
}
impl Encodable for Query {
	fn encode(&self) -> Vec<u8> {
		use Query::*;
		match self {
			DB(f) => f.encode(),
		}
	}
}
impl<R: Read> Decodable<R> for Query {
	fn decode(bytes: &mut R) -> std::io::Result<Self> {
		Ok(Query::DB(Function::decode(bytes)?))
	}
}

#[derive(Debug)]
pub enum FunctionType {
	Table,
	Read,
}
impl FunctionType {}
impl Encodable for FunctionType {
	fn encode(&self) -> Vec<u8> {
		use FunctionType::*;
		let b: u8 = match self {
			Table => 0,
			Read => 1,
		};
		vec![b]
	}
}
impl<R: Read> Decodable<R> for FunctionType {
	fn decode(bytes: &mut R) -> std::io::Result<Self> {
		let discriminant = u8::decode(bytes)?;
		match discriminant {
			0 => Ok(FunctionType::Table),
			1 => Ok(FunctionType::Read),
			_ => Err(io::Error::other("Malformed request content")),
		}
	}
}

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
impl Encodable for Function {
	fn encode(&self) -> Vec<u8> {
		let mut out = Vec::new();
		out.extend_from_slice(&self.ty.encode());
		out.extend_from_slice(&self.args.encode());
		out.extend_from_slice(&self.args.encode());
		out
	}
}
impl<R: Read> Decodable<R> for Function {
	fn decode(bytes: &mut R) -> io::Result<Self> {
		Ok(Function {
			ty: FunctionType::decode(bytes)?,
			args: Vec::<Value>::decode(bytes)?,
			chained: Option::<Function>::decode(bytes)?.map(Box::new),
		})
	}
}

#[derive(Debug)]
pub enum Value {
	String(String),
}
impl Encodable for Value {
	fn encode(&self) -> Vec<u8> {
		let mut out = Vec::new();
		let discriminant: u8 = match self {
			Value::String(_) => 0,
		};
		out.push(discriminant);
		match self {
			Value::String(s) => {
				out.extend_from_slice(&s.encode());
			}
		}
		out
	}
}
impl<R: Read> Decodable<R> for Value {
	fn decode(bytes: &mut R) -> io::Result<Self> {
		let discriminant = u8::decode(bytes)?;
		match discriminant {
			0 => {
				let s = String::decode(bytes)?;
				return Ok(Value::String(s));
			}
			_ => Err(io::Error::other("Malformed request content")),
		}
	}
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
