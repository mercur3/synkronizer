use crate::utils::file_system::*;
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
			_ => Err(()),
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
		let target = &link.target;
		if target.exists() {
			println!(
				"Target file {} exists. Proceeding with conflict resolver.",
				target.display()
			);
			match link.resolver {
				ConflictResolver::Prompt => self.prompt_for_overwrite(link),
				ConflictResolver::Overwrite => self.overwrite_link(link),
				ConflictResolver::DoNothing => Ok(()),
			}
		}
		else {
			println!(
				"Target file {} does not exists. Creating new link.",
				target.display()
			);
			match unix::symlink(&link.src, &link.target) {
				Ok(_) => Ok(()),
				Err(e) => {
					eprintln!("{}", e);
					Err(String::from("Cannot link"))
				}
			}
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
	let target = to_abs_path(target).expect(&format!("Cannot resolve path to {}", target));

	DirContent {
		dir: target,
		resolver,
		reader: fs::read_dir(src).expect(&format!("Cannot open dir {}", src.display())),
	}
}

#[cfg(test)]
mod test {
	use super::*;
	// use std::process::{Command, Stdio};

	// fn setup_target_dir() {
	// 	Command::new("../app/tests/x/script.sh")
	// 		.stdout(Stdio::null())
	// 		.stdin(Stdio::null())
	// 		.stderr(Stdio::null())
	// 		.output()
	// 		.unwrap();
	// }

	struct TestDir<'a> {
		name: PathBuf,
		files: Vec<TestFile<'a>>,
	}

	struct TestFile<'a> {
		name: PathBuf,
		content: &'a str,
	}

	impl<'a> TestDir<'a> {
		fn base(name: &'a str, parent: &'a Path) -> Self {
			TestDir {
				name: parent.join(name),
				files: vec![],
			}
		}

		fn add_file(&mut self, name: &'a str, content: &'a str) {
			let file = TestFile {
				name: self.name.join(name),
				content,
			};
			self.files.push(file);
		}

		fn build(self) -> io::Result<()> {
			fs::create_dir(&self.name)?;
			for el in self.files {
				el.build()?;
			}

			Ok(())
		}
	}

	impl<'a> TestFile<'a> {
		fn build(self) -> io::Result<()> {
			let mut file = fs::File::create(&self.name)?;
			file.write_all(self.content.as_bytes())?;
			Ok(())
		}
	}

	fn setup_dirs(src: &Path, target: &Path) -> io::Result<()> {
		if src.exists() {
			fs::remove_dir_all(src)?;
		}
		if target.exists() {
			fs::remove_dir_all(target)?;
		}

		fs::create_dir(src)?;
		fs::create_dir(target)?;

		// src dir
		let mut alpha = TestDir::base("alpha", src);
		alpha.add_file("a", "Ut laoreet tristique lectus eget egestas.");
		alpha.add_file("b", "");
		alpha.add_file("c", "Donec et mauris in risus convallis tempus.");
		alpha.build()?;

		let mut beta = TestDir::base("beta", src);
		beta.add_file("irrelevant.txt", "");
		beta.build()?;

		let mut gamma = TestDir::base("gamma", src);
		gamma.add_file("a", "");
		gamma.build()?;

		let files = [
			TestFile {
				name: src.join("1"),
				content: "",
			},
			TestFile {
				name: src.join("2"),
				content: "qwerty",
			},
			TestFile {
				name: src.join("3"),
				content: "In hac habitasse platea dictumst.",
			},
		];
		for el in files {
			el.build()?;
		}

		// target dir
		let files = [
			TestFile {
				name: target.join("1"),
				content: "",
			},
			TestFile {
				name: target.join("2"),
				content: "qwerty",
			},
		];
		for el in files {
			el.build()?;
		}
		unix::symlink(src.join("alpha"), &target.join("alpha"))?;
		unix::symlink(src.join("3"), &target.join("3"))?;

		Ok(())
	}

	fn paths() -> (String, String) {
		let src = expand_tilde("~/code/personal/synkronizer/app/tests/x/src");
		let target = expand_tilde("~/code/personal/synkronizer/app/tests/x/target");

		(src.into_owned(), target.into_owned())
	}

	#[test]
	fn test_link() {
		let (src_path, target_path) = paths();
		let src = Path::new(&src_path);
		let target = Path::new(&target_path);

		setup_dirs(src, target).unwrap();
		link_with_do_nothing_conflict_resolver(&src_path, &target_path);

		setup_dirs(src, target).unwrap();
		link_with_overwrite_conflict_resolver(&src_path, &target_path);
	}

	fn link_with_do_nothing_conflict_resolver(src_path: &str, target_path: &str) {
		let do_nothing_linker = CliLinker::new();
		let dir_reader = sync(
			&Path::new(src_path),
			target_path,
			ConflictResolver::DoNothing,
		);

		dir_reader.for_each(|el| do_nothing_linker.link(&el).unwrap());

		let f1 = fs::read_link(Path::new(&format!("{}{}", target_path, 1)));
		let f2 = fs::read_link(Path::new(&format!("{}{}", target_path, 2)));
		let f3 = fs::read_link(Path::new(&format!("{}/{}", target_path, 3))).unwrap();
		let d1 = fs::read_link(Path::new(&format!("{}/{}", target_path, "alpha"))).unwrap();
		let d2 = fs::read_link(Path::new(&format!("{}/{}", target_path, "beta"))).unwrap();
		let d3 = fs::read_link(Path::new(&format!("{}/{}", target_path, "gamma"))).unwrap();

		matches!(f1, Err(_));
		matches!(f2, Err(_));
		assert_eq!(&f3, Path::new(&format!("{}/{}", src_path, 3)));
		assert_eq!(&d1, Path::new(&format!("{}/{}", src_path, "alpha")));
		assert_eq!(&d2, Path::new(&format!("{}/{}", src_path, "beta")));
		assert_eq!(&d3, Path::new(&format!("{}/{}", src_path, "gamma")));
	}

	fn link_with_overwrite_conflict_resolver(src_path: &str, target_path: &str) {
		let overwrite_linker = CliLinker::new();
		let vec = sync(
			&Path::new(src_path),
			target_path,
			ConflictResolver::Overwrite,
		);

		vec.for_each(|el| overwrite_linker.link(&el).unwrap());

		let f1 = fs::read_link(Path::new(&format!("{}/{}", target_path, 1))).unwrap();
		let f2 = fs::read_link(Path::new(&format!("{}/{}", target_path, 2))).unwrap();
		let f3 = fs::read_link(Path::new(&format!("{}/{}", target_path, 3))).unwrap();
		let d1 = fs::read_link(Path::new(&format!("{}/{}", target_path, "alpha"))).unwrap();
		let d2 = fs::read_link(Path::new(&format!("{}/{}", target_path, "beta"))).unwrap();
		let d3 = fs::read_link(Path::new(&format!("{}/{}", target_path, "gamma"))).unwrap();

		assert_eq!(&f1, Path::new(&format!("{}/{}", src_path, "1")));
		assert_eq!(&f2, Path::new(&format!("{}/{}", src_path, "2")));
		assert_eq!(&f3, Path::new(&format!("{}/{}", src_path, "3")));
		assert_eq!(&d1, Path::new(&format!("{}/{}", src_path, "alpha")));
		assert_eq!(&d2, Path::new(&format!("{}/{}", src_path, "beta")));
		assert_eq!(&d3, Path::new(&format!("{}/{}", src_path, "gamma")));
	}
}
