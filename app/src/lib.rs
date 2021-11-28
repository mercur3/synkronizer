use libsynkronizer::sync::*;
use std::cell::RefCell;
use std::fs;
use std::io::{self, Stdin, Stdout, Write};

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

	fn prompt_for_overwrite(&self, link: &Link) -> Result<(), String> {
		let target = &link.target;

		loop {
			let msg = &format!("Do you want to overwrite {} [y/N]? ", target.display());
			let input = self.prompt(msg);

			match input.trim() {
				"y" | "Y" => return self.overwrite_link(link),
				"n" | "N" | "" => return Ok(()),
				x => eprintln!("Unknown parameter {}", x),
			}
		}
	}
}
