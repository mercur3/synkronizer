pub mod utils;

use std::fs;
use std::io;
use std::io::BufRead;
pub use std::path::Path;

pub struct App {
	home: Box<Path>,
	config: Box<Path>,
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
	let mut home = String::from("");
	let mut config = String::from("");

	let file = io::BufReader::new(fs::File::open(path).expect("cannot open file"));
	for line in file.lines() {
		let args = line.unwrap();

		let args: Vec<&str> = args
			.split_ascii_whitespace()
			.filter(|x| x.len() != 0)
			.collect();

		if args.len() == 0 || args[0] == "#" {
			continue;
		}
		if args.len() != 3 {
			panic!("Line must have length == 3 or should start with #");
		}
		if args[1] == "=" {
			panic!("Missing =");
		}

		let arg0 = args[0].to_lowercase();
		if arg0 == crate::HOME_KEYWORD {
			home = args[2].to_string();
		}
		else if arg0 == crate::CONFIG_KEYWORD {
			config = args[2].to_string();
		}
	}

	return (home, config);
}
