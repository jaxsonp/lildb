use super::*;

#[test]
fn check_api_version_compat() {
	use super::check_api_version_compat;
	let v: ApiVersion = ApiVersion(1, 4, 3);

	assert!(check_api_version_compat(v, ApiVersion(1, 4, 0)));
	assert!(check_api_version_compat(v, ApiVersion(1, 4, 3)));
	assert!(check_api_version_compat(v, ApiVersion(1, 0, 9)));
	assert!(check_api_version_compat(v, ApiVersion(1, 3, 9999)));

	assert!(!check_api_version_compat(v, ApiVersion(1, 5, 0)));
	assert!(!check_api_version_compat(v, ApiVersion(1, 4, 4)));
	assert!(!check_api_version_compat(v, ApiVersion(0, 12, 0)));
	assert!(!check_api_version_compat(v, ApiVersion(2, 0, 0)));
}
