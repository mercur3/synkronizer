use std::fs;
use std::path::Path;

pub fn sync(src: &Path, target: &Path) {
	println!("Syncing..");
	println!("Base dir: {}", src.display());
	println!("Target dir: {}", target.display());

	for i in fs::read_dir(src).unwrap() {
		let file = &i.unwrap().path();
		println!("{}", file.display());
	}
}
