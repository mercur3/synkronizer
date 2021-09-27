pub mod utils;

use std::fs::File;
use std::io::{self, BufRead};
pub use std::path::{Path, PathBuf};

pub struct App {
	pub home: PathBuf,
	pub config: PathBuf,
}

static HOME_KEYWORD: &str = "home";
static CONFIG_KEYWORD: &str = "config";

impl App {
	pub fn from_config_file(path: &Path) -> Self {
		let (home, config) = crate::parse_file(path);
		if home.is_empty() {
			panic!("No home");
		}
		if config.is_empty() {
			panic!("No config");
		}

		let home = utils::file_system::to_abs_path(home);
		let config = utils::file_system::to_abs_path(config);

		return App { home, config };
	}
}

fn parse_file(path: &Path) -> (String, String) {
	let mut home = String::default();
	let mut config = String::default();

	let file = io::BufReader::new(File::open(path).expect("cannot open file"));
	for line in file.lines() {
		let args = line.unwrap_or_default();
		let args = args.trim();
		if args.is_empty() || args.starts_with('#') {
			continue;
		}

		let index = args
			.find('=')
			.expect(format!("Missing `=`\nLine: {}", args).as_str());
		let left = args[..index].trim().to_lowercase();
		let right = args[index + 1..].trim().to_lowercase();

		if left.is_empty() {
			panic!("Missing left hand side");
		}
		if right.is_empty() {
			panic!("Missing right hand side");
		}

		if left == crate::HOME_KEYWORD {
			home = right;
		}
		else if left == crate::CONFIG_KEYWORD {
			config = right;
		}
		else {
			eprintln!("Keyword {} is not known", left);
			eprintln!("Line: {}", args);
			panic!("Unknown keyword");
		}
	}

	return (home, config);
}