use std::env;

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	println!("cargo::rustc-env=TEST_OUTPUT_DIR={}/test_output/", out_dir);
}
