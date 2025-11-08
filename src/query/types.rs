#[derive(Debug, PartialEq, Eq)]
pub enum Type {
	None,
	DB,
	/// A reference to a table/index/whatever that may or may not exist
	Object,
	/// Data read from a table/index
	Records,

	StringLiteral,
}
