#[cfg(test)]
pub mod testing;
#[cfg(test)]
pub mod tests;

use std::fs;

use lildb_api::ApiVersion;

use crate::*;

/// Returns true if the provided api version is semver compatible with the api that this was compiled with
///
/// ```
/// # use lildb_server_core::utils::check_api_version_compat;
/// use lildb_api::ApiVersion;
///
/// const V: ApiVersion = ApiVersion(1, 4, 3);
/// assert!(check_api_version_compat(V, ApiVersion(1, 4, 0)));
/// assert!(check_api_version_compat(V, ApiVersion(1, 0, 9)));
///
/// assert!(!check_api_version_compat(V, ApiVersion(1, 5, 0)));
/// assert!(!check_api_version_compat(V, ApiVersion(1, 4, 4)));
/// assert!(!check_api_version_compat(V, ApiVersion(0, 12, 0)));
/// ```
pub fn check_api_version_compat(me: ApiVersion, other: ApiVersion) -> bool {
	(me.0 == other.0) && ((me.1 > other.1) || (me.1 == other.1 && me.2 >= other.2))
}

/// Creates or asserts the existence of the proper directory structure
pub(crate) fn validate_dirs(config: &Config) -> Result<(), ServerError> {
	let dirs_to_validate = vec![config.data_path.clone(), config.db_path()];
	for dir_path in dirs_to_validate.into_iter() {
		if !dir_path.exists() {
			log::warn!("Creating directory: {}", dir_path.display());
			fs::create_dir(dir_path)?;
		} else if !dir_path.is_dir() {
			return Err(ServerError::Config(format!(
				"Expected directory at \"{}\", found file",
				dir_path.display()
			)));
		}
	}
	log::debug!("Directory structure validated");
	Ok(())
}
