use libsynkronizer::app::App;
use libsynkronizer::sync::{CliLinker, Linker};
use std::env;
use std::path::Path;

const DEFAULT_PATH: &str = "config.txt";
const HELP_LONG: &str = "--help";
const HELP_SHORT: &str = "-h";

fn run(path: &Path) {
	// TODO uncomment when release
	// let app = App::from_config_file(path);
	// let linker = CliLinker::new();

	// let dir_reader = app.sync_home();
	// for l in dir_reader {
	// 	linker.link(&l).unwrap();
	// }

	// let dir_reader = app.sync_config();
	// for l in dir_reader {
	// 	linker.link(&l).unwrap();
	// }
}

fn print_help() {
	println!(
		r#"
	synkronizer

	NAME
		synkronizer - Like GNU Stow, but written in Rust and with 0 dependencies.

	SYNOPSIS
		./synkronizer [OPTION | FILE]

	DESCRIPTION
		Sync config files from a git repo by using symlinks.
		If no argument is passed uses default path "config.txt".

		-h, --help
			print this help message

		FILE
			path to config file.

	AUTHOR
		Written by Andri Reveli
	"#
	);
}

fn main() {
	let mut args = env::args().skip(1);

	match args.size_hint() {
		(0, _) => {
			run(Path::new(DEFAULT_PATH));
		}
		(1, _) => {
			let argument = args.next().unwrap();

			if argument == HELP_LONG || argument == HELP_SHORT {
				print_help();
			}
			else {
				run(Path::new(&argument));
			}
		}
		_ => eprintln!("Unsupported number of command line arguments"),
	}
}
