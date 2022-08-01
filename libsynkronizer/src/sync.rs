use crate::utils::file_system;
use std::cell::RefCell;
use std::fs;
use std::io::{self, Stdin, Stdout, Write};
use std::os::unix::fs as unix;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone)]
pub enum ConflictResolver {
	Prompt,
	Overwrite,
	DoNothing,
}

impl FromStr for ConflictResolver {
	type Err = ();
	fn from_str(text: &str) -> Result<Self, Self::Err> {
		match text.to_lowercase().as_ref() {
			"prompt" => Ok(ConflictResolver::Prompt),
			"overwrite" => Ok(ConflictResolver::Overwrite),
			"do_nothing" => Ok(ConflictResolver::DoNothing),
			x => Err(()),
		}
	}
}

pub struct Link {
	pub src: PathBuf,
	pub target: PathBuf,
	pub resolver: ConflictResolver,
}

pub trait Linker {
	fn log(&self, msg: &str);

	fn log_err(&self, msg: &str);

	fn prompt(&self, msg: &str) -> String;

	fn prompt_for_overwrite(&self, link: &Link) -> Result<(), String>;

	fn link(&self, link: &Link) -> Result<(), String> {
		match link.target.exists() {
			true => match link.resolver {
				ConflictResolver::Prompt => self.prompt_for_overwrite(link),
				ConflictResolver::Overwrite => self.overwrite_link(link),
				ConflictResolver::DoNothing => Ok(()),
			},
			false => match unix::symlink(&link.src, &link.target) {
				Ok(_) => Ok(()),
				_ => Err(String::from("Cannot link")),
			},
		}
	}

	fn overwrite_link(&self, link: &Link) -> Result<(), String> {
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

		match unix::symlink(src, target) {
			Ok(()) => Ok(()),
			Err(_) => Err(format!(
				"Catastrophic error\nsrc: {}\ntarget: {}",
				src.display(),
				target.display()
			)),
		}
	}
}

pub struct CliLinker {
	stdin: RefCell<Stdin>,
	stdout: RefCell<Stdout>,
}

impl CliLinker {
	pub fn new() -> CliLinker {
		CliLinker {
			stdin: RefCell::new(io::stdin()),
			stdout: RefCell::new(io::stdout()),
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

		buffer
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

pub struct DirContent {
	dir: PathBuf,
	resolver: ConflictResolver,
	reader: fs::ReadDir,
}

impl Iterator for DirContent {
	type Item = Link;

	fn next(&mut self) -> Option<Self::Item> {
		match self.reader.next() {
			Some(entry) => {
				let entry = entry.unwrap();
				let original_location = entry.path();
				let file_name = &entry.file_name();
				let new_location = self.dir.clone().join(file_name);

				Some(Link {
					src: original_location,
					target: new_location,
					resolver: self.resolver.clone(),
				})
			}
			None => None,
		}
	}
}

/// Syncs files in the `src` to `target`.
/// `src` has the meaning the path where we will get the link from
/// `target` has the meaning where the link will point to
pub fn sync(src: &Path, target: &str, resolver: ConflictResolver) -> DirContent {
	let target = file_system::to_abs_path(target);

	DirContent {
		dir: target,
		resolver,
		reader: fs::read_dir(src).expect(&format!("Cannot open dir {}", src.display())),
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::utils::file_system::expand_tilde;
	use std::process::{Command, Stdio};

	const SRC_PATH: &str = "/home/andri/code/personal/synkronizer/app/tests/x/src";
	const TARGET_PATH: &str = "/home/andri/code/personal/synkronizer/app/tests/x/target";

	fn setup_target_dir() {
		Command::new("../app/tests/x/script.sh")
			.stdout(Stdio::null())
			.stdin(Stdio::null())
			.stderr(Stdio::null())
			.output()
			.unwrap();
	}

	fn base_paths() -> (String, String) {
		let target_base = String::from("../app/tests/x/target/");
		let src_base = expand_tilde("~/code/personal/synkronizer/app/tests/x/src/");

		(target_base, src_base.into_owned())
	}

	#[test]
	fn test_link() {
		link_with_do_nothing_conflict_resolver();
		link_with_overwrite_conflict_resolver();
	}

	fn link_with_do_nothing_conflict_resolver() {
		setup_target_dir();

		let do_nothing_linker = CliLinker::new();
		dbg!("src: {}", SRC_PATH);
		dbg!("src: {}", TARGET_PATH);
		let dir_reader = sync(
			&Path::new(SRC_PATH),
			TARGET_PATH,
			ConflictResolver::DoNothing,
		);

		for l in dir_reader {
			do_nothing_linker.link(&l).unwrap();
		}

		let (target_base, src_base) = base_paths();
		let f1 = fs::read_link(Path::new(&format!("{}{}", target_base, 1)));
		let f2 = fs::read_link(Path::new(&format!("{}{}", target_base, 2)));
		let f3 = fs::read_link(Path::new(&format!("{}{}", target_base, 3))).unwrap();
		let d1 = fs::read_link(Path::new(&format!("{}{}", target_base, "alpha"))).unwrap();
		let d2 = fs::read_link(Path::new(&format!("{}{}", target_base, "beta"))).unwrap();
		let d3 = fs::read_link(Path::new(&format!("{}{}", target_base, "gamma"))).unwrap();

		matches!(f1, Err(_));
		matches!(f2, Err(_));
		assert_eq!(&f3, Path::new(&format!("{}{}", src_base, 3)));
		assert_eq!(&d1, Path::new(&format!("{}{}", src_base, "alpha")));
		assert_eq!(&d2, Path::new(&format!("{}{}", src_base, "beta")));
		assert_eq!(&d3, Path::new(&format!("{}{}", src_base, "gamma")));
	}

	fn link_with_overwrite_conflict_resolver() {
		setup_target_dir();

		let overwrite_linker = CliLinker::new();
		let vec = sync(
			&Path::new(SRC_PATH),
			TARGET_PATH,
			ConflictResolver::Overwrite,
		);

		for l in vec {
			overwrite_linker.link(&l).unwrap();
		}

		let (target_base, src_base) = base_paths();
		let f1 = fs::read_link(Path::new(&format!("{}{}", target_base, 1))).unwrap();
		let f2 = fs::read_link(Path::new(&format!("{}{}", target_base, 2))).unwrap();
		let f3 = fs::read_link(Path::new(&format!("{}{}", target_base, 3))).unwrap();
		let d1 = fs::read_link(Path::new(&format!("{}{}", target_base, "alpha"))).unwrap();
		let d2 = fs::read_link(Path::new(&format!("{}{}", target_base, "beta"))).unwrap();
		let d3 = fs::read_link(Path::new(&format!("{}{}", target_base, "gamma"))).unwrap();

		assert_eq!(&f1, Path::new(&format!("{}{}", src_base, "1")));
		assert_eq!(&f2, Path::new(&format!("{}{}", src_base, "2")));
		assert_eq!(&f3, Path::new(&format!("{}{}", src_base, "3")));
		assert_eq!(&d1, Path::new(&format!("{}{}", src_base, "alpha")));
		assert_eq!(&d2, Path::new(&format!("{}{}", src_base, "beta")));
		assert_eq!(&d3, Path::new(&format!("{}{}", src_base, "gamma")));
	}
}

