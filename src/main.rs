use std::env;
use std::path::Path;
use synkronizer::*;

const DEFAULT_PATH: &str = "config.txt";
const HELP_COMMAND: &str = "--help";

fn run(path: &Path) {
	// TODO uncomment when release
	// let app = App::from_config_file(path);
	// app.sync_home();
	// app.sync_config();
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

		--help
			print this help message

		FILE
			path to config file.

	AUTHOR
		Written by Andri Reveli
	"#
	);
}

fn main() {
	let args: Vec<_> = env::args().skip(1).collect();

	match args.len() {
		0 => {
			run(Path::new(DEFAULT_PATH));
		}
		1 => {
			if args[0] == HELP_COMMAND {
				print_help();
			}
			else {
				run(Path::new(&args[0]));
			}
		}
		_ => eprintln!("Unsupported number of command line arguments"),
	}
}
