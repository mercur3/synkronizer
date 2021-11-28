pub mod sync;
pub mod utils;

use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

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
		if home.is_empty() {
			panic!("No home");
		}
		if config.is_empty() {
			panic!("No config");
		}

		let home = utils::file_system::to_abs_path(&home);
		let config = utils::file_system::to_abs_path(&config);

		return App {
			home,
			config,
			resolver,
		};
	}

	fn parse_file(path: &Path) -> (Cow<str>, Cow<str>, sync::ConflictResolver) {
		let mut home = Cow::Borrowed("");
		let mut config = Cow::Borrowed("");
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

			if left.is_empty() {
				panic!("Missing left hand side");
			}
			if right.is_empty() {
				panic!("Missing right hand side");
			}

			match left.as_str() {
				crate::HOME_KEYWORD => home = Cow::Owned(right),
				crate::CONFIG_KEYWORD => config = Cow::Owned(right),
				crate::CONFILCT_RESOLVER_KEYWORD => {
					resolver = sync::ConflictResolver::from(right.as_ref())
				}
				_ => {
					eprintln!("Keyword {} is not known", left);
					eprintln!("Line: {}", args);
					panic!("Unknown keyword");
				}
			}
		}

		return (home, config, resolver);
	}

	pub fn sync_home(&self) -> Vec<sync::Link> {
		let src = &self.home;
		let target = &utils::file_system::to_abs_path("~");
		let resolver = &self.resolver;

		return sync::sync(src, target, resolver.clone());
	}

	pub fn sync_config(&self) -> Vec<sync::Link> {
		let src = &self.config;
		let target = &utils::file_system::to_abs_path("~/.config");
		let resolver = &self.resolver;

		return sync::sync(src, target, resolver.clone());
	}
}
