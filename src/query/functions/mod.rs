#![allow(non_upper_case_globals)]
mod definitions;

use std::fmt::Debug;

use super::Type;

pub use definitions::*;

#[derive(Debug, PartialEq, Eq)]
pub struct FunctionDef {
	pub name: &'static str,
	pub positional_args: &'static [Type],
	/// Type that this function is called on
	pub object_type: Type,
	/// Type that this function returns
	pub return_type: Type,
}

pub const FUNCTIONS: &[FunctionDef] = &[createFunction, ensureExistsFunction, deleteFunction];

/// Find function by name
pub fn find_function(name: &String) -> Option<&'static FunctionDef> {
	for f in FUNCTIONS.iter() {
		if f.name == name {
			return Some(f);
		}
	}
	None
}
