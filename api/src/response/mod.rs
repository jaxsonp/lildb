mod query_res;

pub use query_res::QueryResult;

/// Response from the server
#[derive(Debug)]
pub enum Response {
	/// Positive acknowledgment
	Ok,
	/// Response to a query
	QueryResponse(QueryResult),
	/// Error while serving request
	Error(String),
}
