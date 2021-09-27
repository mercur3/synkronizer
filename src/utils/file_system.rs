pub use std::fs;
use std::path::PathBuf;

pub fn expand_tilde(s: String) -> String {
	if s.starts_with('~') {
		return s.replacen('~', &std::env::var("HOME").unwrap(), 1);
	}
	return s;
}

pub fn to_abs_path(s: String) -> PathBuf {
	let p = expand_tilde(s);
	return fs::canonicalize(p).unwrap();
}

#[test]
fn file_exists() {
	assert!(fs::canonicalize(expand_tilde(String::from("~/code/linux-configs/"))).is_ok());
}

#[test]
fn home_is_correct() {
	assert_eq!(expand_tilde(String::from("~")), "/home/andri");
}
