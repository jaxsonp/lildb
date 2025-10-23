use lildb_api::ApiVersion;

#[cfg(test)]
pub mod testing;
#[cfg(test)]
pub mod tests;

/// Returns true if the provided api version is semver compatible with the api that this was compiled with
///
/// ```
/// # use lildb_server_core::util::check_api_version_compat;
/// use lildb_api::ApiVersion;
///
/// const V: ApiVersion = (1, 4, 3);
/// assert!(check_api_version_compat(V, (1, 4, 0)));
/// assert!(check_api_version_compat(V, (1, 0, 9)));
///
/// assert!(!check_api_version_compat(V, (1, 5, 0)));
/// assert!(!check_api_version_compat(V, (1, 4, 4)));
/// assert!(!check_api_version_compat(V, (0, 12, 0)));
/// ```
pub fn check_api_version_compat(me: ApiVersion, other: ApiVersion) -> bool {
	(me.0 == other.0) && ((me.1 > other.1) || (me.1 == other.1 && me.2 >= other.2))
}
