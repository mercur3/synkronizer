use libsynkronizer::app::App;
use libsynkronizer::sync::ConflictResolver;
use std::path::Path;

const BASE_CORRECT_PATH: &str = "./tests/files/correct/config{}.txt";
const BASE_INVALID_PATH: &str = "./tests/files/invalid/err{}.txt";

#[test]
fn file_is_correct() {
	let path = BASE_CORRECT_PATH.replace("{}", "");
	match std::fs::canonicalize(&path) {
		Ok(path) => {
			let base_config = App::from_config_file(&path);

			let expected_resolvers = vec![
				ConflictResolver::Prompt,
				ConflictResolver::Prompt,
				ConflictResolver::DoNothing,
				ConflictResolver::Overwrite,
				ConflictResolver::DoNothing,
				ConflictResolver::DoNothing,
			];
			let mut actual_resolvers = Vec::with_capacity(6);
			actual_resolvers.push(base_config.resolver);

			for i in 1..=5 {
				let file_name = BASE_CORRECT_PATH.replace("{}", &i.to_string());
				println!("config file: {file_name}");
				let path = Path::new(&file_name);
				let x = App::from_config_file(path);

				actual_resolvers.push(x.resolver);

				assert_eq!(base_config.home, x.home);
				assert_eq!(base_config.config, x.config);
			}

			for i in 0..6 {
				let expected = &expected_resolvers[i];
				let _actual = &actual_resolvers[i];
				matches!(expected, _actual);
			}
		},
		Err(x) => eprintln!("\n\nError:\n{x}\n\n"),
	}
}

#[test]
fn parses_whitespace_path() {
	let p = BASE_CORRECT_PATH.replace("{}", "6");
	let x = App::from_config_file(Path::new(&p));

	assert_eq!(x.config, x.home);
	assert!(x.home.is_dir());
	assert!(x.config.is_dir());
	matches!(x.resolver, ConflictResolver::Prompt);
}

#[test]
#[should_panic(expected = "Unknown keyword")]
fn uknown_keyword() {
	let p = BASE_INVALID_PATH.replace("{}", "1");
	App::from_config_file(Path::new(&p));
}

#[test]
#[should_panic(expected = "Missing `=`")]
fn missing_equals_sign() {
	let p = BASE_INVALID_PATH.replace("{}", "2");
	App::from_config_file(Path::new(&p));
}

#[test]
#[should_panic(expected = "No config")]
fn no_config() {
	let p = BASE_INVALID_PATH.replace("{}", "3");
	App::from_config_file(Path::new(&p));
}

#[test]
#[should_panic(expected = "No home")]
fn no_home() {
	let p = BASE_INVALID_PATH.replace("{}", "4");
	App::from_config_file(Path::new(&p));
}

#[test]
#[should_panic(expected = "No such file or directory")]
fn invalid_path() {
	let p = BASE_INVALID_PATH.replace("{}", "5");
	App::from_config_file(Path::new(&p));
}

#[test]
#[should_panic(expected = "Cannot instantiate a ConflictResolver.")]
fn wrong_resolver_option() {
	let p = BASE_INVALID_PATH.replace("{}", "6");
	App::from_config_file(Path::new(&p));
}
