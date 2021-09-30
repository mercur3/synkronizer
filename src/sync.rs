use std::fs;
use std::io;
use std::os::unix::fs as unix;
use std::path::Path;

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

/// Syncs files in the `src` to `target`.
/// `src` has the meaning the path where we will get the link from
/// `target` has the meaning where the link will point to
pub fn sync(src: &Path, target: &Path, resolve: &ConflictResolver) {
	println!("Syncing..");
	println!("Base dir: {}", src.display());
	println!("Target dir: {}", target.display());

	for i in fs::read_dir(src).unwrap() {
		let entry = i.unwrap();
		let original_location = &entry.path();
		let file_name = &entry.file_name();
		let new_location = &target.clone().join(file_name);

		println!(
			"{} -> {}",
			original_location.display(),
			new_location.display()
		);
		// link(original_location, new_location).expect("Unable to perform this action");
	}
}

fn link(src: &Path, target: &Path, resolver: ConflictResolver) -> io::Result<()> {
	return match target.exists() {
		true => match resolver {
			ConflictResolver::Prompt => prompt(src, target),
			ConflictResolver::Overwrite => overwrite_link(src, target),
			ConflictResolver::DoNothing => Ok(()),
		},
		false => unix::symlink(src, target),
	};
}

fn prompt(src: &Path, target: &Path) -> io::Result<()> {
	loop {
		println!("Do you want to overwrite {} [y/N]?", target.display());

		let mut input = String::default();
		io::stdin().read_line(&mut input);
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
		fs::remove_file(target);
	}
	else if target.is_dir() {
		fs::remove_dir_all(target);
	}
	else {
		eprintln!("Catastrophic error");
		eprintln!("src: {}", src.display());
		eprintln!("target: {}", target.display());
	}
	return unix::symlink(src, target);
}

// TODO
//          TESTS          //
