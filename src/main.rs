use std::path::Path;
use synkronizer::*;

fn main() {
	// let app = App::from_config_file(Path::new("config.txt"));
	// app.sync_home();
	// app.sync_config();

	// FIXME convert to unit test
	// let src = &Path::new("/home/andri/code/synkronizer/tests/x/src");
	// let target = &Path::new("/home/andri/code/synkronizer/tests/x/target");
	// let resolve = &sync::ConflictResolver::Prompt;
	// sync::sync(src, target, resolve);
	std::process::Command::new("tests/x/script.sh")
		.output()
		.unwrap();
}
