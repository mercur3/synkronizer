pub use std::fs;
pub use std::path::Path;

pub fn expand_tilde(s: String) -> String {
	if s.starts_with('~') {
		return s.replacen('~', &std::env::var("HOME").unwrap(), 1);
	}
	return s;
}

pub fn to_abs_path(s: String) -> Box<Path> {
	return fs::canonicalize(expand_tilde(s)).unwrap().into_boxed_path();
}

#[test]
fn file_exists() {
	assert!(fs::canonicalize(expand_tilde(String::from("~/code/linux-configs/"))).is_ok());
}

#[test]
fn home_is_correct() {
	assert_eq!(expand_tilde(String::from("~")), "/home/andri");
}
