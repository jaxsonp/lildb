/// Request sent to the server
///
/// Will eventually populate with authentication and session management messages
pub enum Request {
	Query(String),
	Exit,
}

/// Response from the server
pub enum ServerResponse {
	/// Positive acknowledgment
	Ok,
	/// Response to a query
	QueryResponse(QueryResult),
	/// Error while serving request
	Error(String),
}

/// Data returned from a query
pub struct QueryResult {}
