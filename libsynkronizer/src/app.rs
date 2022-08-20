use crate::sync;
use crate::utils::file_system::to_abs_path;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::str::FromStr;

const HOME_KEYWORD: &str = "home";
const CONFIG_KEYWORD: &str = "config";
const CONFILCT_RESOLVER_KEYWORD: &str = "conflict_resolver";

pub struct App {
	pub home: PathBuf,
	pub config: PathBuf,
	pub resolver: sync::ConflictResolver,
}

impl App {
	pub fn from_config_file(path: &Path) -> Self {
		let (home, config, resolver) = App::parse_file(path);

		assert!(!home.is_empty(), "No home");
		assert!(!config.is_empty(), "No config");

		let home = to_abs_path(&home).expect(&format!("Cannot resolve absolute path to {}", home));
		let config =
			to_abs_path(&config).expect(&format!("Cannot resolve absolute path to {}", config));

		App {
			home,
			config,
			resolver,
		}
	}

	fn parse_file(path: &Path) -> (String, String, sync::ConflictResolver) {
		let mut home = String::new();
		let mut config = String::new();
		let mut resolver = sync::ConflictResolver::Prompt;

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

			assert!(!left.is_empty(), "Missing left hand side");
			assert!(!right.is_empty(), "Missing right hand side");

			match left.as_str() {
				HOME_KEYWORD => home = String::from(right),
				CONFIG_KEYWORD => config = String::from(right),
				CONFILCT_RESOLVER_KEYWORD => {
					resolver = sync::ConflictResolver::from_str(right.as_ref())
						.expect("Cannot instantiate a ConflictResolver.")
				}
				_ => {
					eprintln!("Keyword {} is not known", left);
					eprintln!("Line: {}", args);
					panic!("Unknown keyword");
				}
			}
		}

		(home, config, resolver)
	}

	pub fn sync_home(&self) -> sync::DirContent {
		let src = &self.home;
		let target = "~";
		let resolver = &self.resolver;

		sync::sync(src, target, resolver.clone())
	}

	pub fn sync_config(&self) -> sync::DirContent {
		let src = &self.config;
		let target = "~/.config";
		let resolver = &self.resolver;

		sync::sync(src, target, resolver.clone())
	}
}
