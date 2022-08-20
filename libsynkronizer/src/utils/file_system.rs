use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

pub fn expand_tilde(s: &str) -> Cow<str> {
	match s.starts_with('~') {
		true => Cow::Owned(s.replacen('~', &std::env::var("HOME").unwrap(), 1)),
		false => Cow::Borrowed(s),
	}
}

pub fn to_abs_path(s: &str) -> std::io::Result<PathBuf> {
	let p = expand_tilde(s);
	fs::canonicalize(p.as_ref())
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn file_exists() {
		let _ = to_abs_path("~/code/linux-configs/").unwrap();
	}

	#[test]
	fn home_is_correct() {
		let lhs = expand_tilde("~").into_owned();
		let rhs = std::env::var("HOME").unwrap();
		assert_eq!(lhs, rhs);
	}
}
