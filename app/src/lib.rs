use libsynkronizer::sync::*;
use std::cell::RefCell;
use std::fs;
use std::io::{self, Stdin, Stdout, Write};
use std::os::unix::fs as unix;

struct CliLinker {
	stdin: RefCell<Stdin>,
	stdout: RefCell<Stdout>,
}

impl CliLinker {
	pub fn new() -> CliLinker {
		return CliLinker {
			stdin: RefCell::new(io::stdin()),
			stdout: RefCell::new(io::stdout()),
		};
	}
}

fn overwrite_link(link: &Link) -> Result<(), String> {
	let src = &link.src;
	let target = &link.target;

	if target.is_file() {
		fs::remove_file(target).unwrap();
	}
	else if link.target.is_dir() {
		fs::remove_dir_all(target).unwrap();
	}
	else {
		return Err(format!(
			"Catastrophic error\nsrc: {}\ntarget: {}",
			src.display(),
			target.display()
		));
	}

	let result = unix::symlink(src, target);
	if result.is_err() {
		return Err(format!(
			"Catastrophic error\nsrc: {}\ntarget: {}",
			src.display(),
			target.display()
		));
	}
	return Ok(());
}

fn prompt_for_overwrite(linker: &CliLinker, link: &Link) -> Result<(), String> {
	let target = &link.target;

	loop {
		let msg = &format!("Do you want to overwrite {} [y/N]? ", target.display());
		let input = linker.prompt(msg);

		match input.trim() {
			"y" | "Y" => return overwrite_link(link),
			"n" | "N" | "" => return Ok(()),
			x => println!("Unknown parameter {}", x),
		}
	}
}

impl Linker for CliLinker {
	fn log(&self, msg: &str) {
		println!("{}", msg);
	}

	fn log_err(&self, msg: &str) {
		eprintln!("{}", msg);
	}

	fn prompt(&self, msg: &str) -> String {
		print!("{}", msg);
		self.stdout.borrow_mut().flush().unwrap();

		let mut buffer = String::with_capacity(128);
		self.stdin
			.borrow_mut()
			.read_line(&mut buffer)
			.expect("Cannot read");

		return buffer;
	}

	fn link(&self, link: &Link) -> Result<(), String> {
		return match link.target.exists() {
			true => match link.resolver {
				ConflictResolver::Prompt => prompt_for_overwrite(self, link),
				ConflictResolver::Overwrite => overwrite_link(link),
				ConflictResolver::DoNothing => Ok(()),
			},
			false => match unix::symlink(&link.src, &link.target) {
				Ok(_) => Ok(()),
				_ => Err(String::from("Cannot link")),
			},
		};
	}
}
