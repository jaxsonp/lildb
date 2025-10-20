/// Request sent to the server
pub enum Request {
	/// List all databases
	ListDBs,
	/// Set current session's active database
	UseDB(String),
	/// Evaluate a query on the current session's active database
	Query(String),
	/// End the session
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
