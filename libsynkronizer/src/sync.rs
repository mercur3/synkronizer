use std::fs;
use std::os::unix::fs as unix;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub enum ConflictResolver {
	Prompt,
	Overwrite,
	DoNothing,
}

impl From<&str> for ConflictResolver {
	fn from(text: &str) -> Self {
		return match text.to_lowercase().as_ref() {
			"prompt" => ConflictResolver::Prompt,
			"overwrite" => ConflictResolver::Overwrite,
			"do_nothing" => ConflictResolver::DoNothing,
			x => panic!(
				"Cannot instantiate a ConflictResolver. Unknown keyword {}",
				x
			),
		};
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
		return match link.target.exists() {
			true => match link.resolver {
				ConflictResolver::Prompt => self.prompt_for_overwrite(link),
				ConflictResolver::Overwrite => self.overwrite_link(link),
				ConflictResolver::DoNothing => Ok(()),
			},
			false => match unix::symlink(&link.src, &link.target) {
				Ok(_) => Ok(()),
				_ => Err(String::from("Cannot link")),
			},
		};
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
}

/// Syncs files in the `src` to `target`.
/// `src` has the meaning the path where we will get the link from
/// `target` has the meaning where the link will point to
pub fn sync(src: &Path, target: &Path, resolve: &ConflictResolver) -> Vec<Link> {
	return fs::read_dir(src)
		.expect(&format!("Cannot open dir {}", src.display()))
		.map(|entry| {
			let entry = entry.unwrap();
			let original_location = entry.path();
			let file_name = &entry.file_name();
			let new_location = target.clone().join(file_name);

			Link {
				src: original_location,
				target: new_location,
				resolver: resolve.clone(),
			}
		})
		.collect();
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::utils::file_system::expand_tilde;
	use std::io::Write;
	use std::os::unix::fs as unix;
	use std::process::{Command, Stdio};

	fn link(src: &Path, target: &Path, resolver: &ConflictResolver) -> io::Result<()> {
		return match target.exists() {
			true => match resolver {
				ConflictResolver::Prompt => prompt(src, target),
				ConflictResolver::Overwrite => overwrite_link(src, target),
				ConflictResolver::DoNothing => {
					println!("ConflictResolver defined as DoNothing, skipping");
					Ok(())
				}
			},
			false => unix::symlink(src, target),
		};
	}

	fn prompt(src: &Path, target: &Path) -> io::Result<()> {
		loop {
			print!("Do you want to overwrite {} [y/N]? ", target.display());
			io::stdout().flush().unwrap();

			let mut input = String::default();
			io::stdin().read_line(&mut input).unwrap();
			let input = input.trim();

			match input {
				"y" | "Y" => return overwrite_link(src, target),
				"n" | "N" | "" => return Ok(()),
				x => println!("Unknown parameter {}", x),
			}
		}
	}

	fn overwrite_link(src: &Path, target: &Path) -> io::Result<()> {
		println!("Replacing {} with a new one", target.display());

		if target.is_file() {
			fs::remove_file(target).unwrap();
		}
		else if target.is_dir() {
			fs::remove_dir_all(target).unwrap();
		}
		else {
			eprintln!("Catastrophic error");
			eprintln!("src: {}", src.display());
			eprintln!("target: {}", target.display());
		}
		return unix::symlink(src, target);
	}

	fn setup_target_dir() {
		Command::new("../app/tests/x/script.sh")
			.stdout(Stdio::null())
			.stdin(Stdio::null())
			.stderr(Stdio::null())
			.output()
			.unwrap();
	}

	fn setup_paths() -> (PathBuf, PathBuf) {
		let src = PathBuf::from("/home/andri/code/personal/synkronizer/app/tests/x/src");
		let target = PathBuf::from("/home/andri/code/personal/synkronizer/app/tests/x/target");
		return (src, target);
	}

	fn base_paths() -> (String, String) {
		let target_base = String::from("../app/tests/x/target/");
		let src_base = expand_tilde("~/code/personal/synkronizer/app/tests/x/src/");
		return (target_base, src_base.into_owned());
	}

	#[test]
	fn link_with_do_nothing_conflict_resolver() {
		setup_target_dir();

		let (src, target) = setup_paths();
		let resolve = ConflictResolver::DoNothing;
		let vec = sync(&src, &target, &resolve);
		for l in vec {
			link(&l.src, &l.target, &l.resolver).unwrap();
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

	#[test]
	fn link_with_overwrite_conflict_resolver() {
		setup_target_dir();

		let (src, target) = setup_paths();
		let resolve = ConflictResolver::Overwrite;
		let vec = sync(&src, &target, &resolve);
		for l in vec {
			link(&l.src, &l.target, &l.resolver).unwrap();
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
