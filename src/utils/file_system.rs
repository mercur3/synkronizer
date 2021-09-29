pub use std::fs;
use std::path::PathBuf;

pub fn expand_tilde(s: &str) -> String {
	if s.starts_with('~') {
		return s.replacen('~', &std::env::var("HOME").unwrap(), 1);
	}
	return String::from(s);
}

pub fn to_abs_path(s: &str) -> PathBuf {
	let p = expand_tilde(s);
	return fs::canonicalize(p).unwrap();
}

#[test]
fn file_exists() {
	assert!(fs::canonicalize(expand_tilde("~/code/linux-configs/")).is_ok());
}

#[test]
fn home_is_correct() {
	assert_eq!(expand_tilde("~"), "/home/andri");
}
