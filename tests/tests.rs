use synkronizer::sync::ConflictResolver;
use synkronizer::{App, Path};

#[test]
fn file_is_correct() {
	let base_path = "./tests/files/correct/config{}.txt";
	let path = base_path.replace("{}", "");
	let path = Path::new(&path);
	let base_config = App::from_config_file(path);

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
		let file_name = base_path.replace("{}", &i.to_string());
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
}

#[test]
fn parses_whitespace_path() {
	let p = Path::new("./tests/files/correct/config6.txt");
	let x = App::from_config_file(p);

	assert_eq!(x.config, x.home);
	assert!(x.home.is_dir());
	assert!(x.config.is_dir());
	matches!(x.resolver, ConflictResolver::Prompt);
}

#[test]
#[should_panic(expected = "Unknown keyword")]
fn uknown_keyword() {
	let p = Path::new("./tests/files/invalid/err1.txt");
	App::from_config_file(p);
}

#[test]
#[should_panic(expected = "Missing `=`")]
fn missing_equals_sign() {
	let p = Path::new("./tests/files/invalid/err2.txt");
	App::from_config_file(p);
}

#[test]
#[should_panic(expected = "No config")]
fn no_config() {
	let p = Path::new("./tests/files/invalid/err3.txt");
	App::from_config_file(p);
}

#[test]
#[should_panic(expected = "No home")]
fn no_home() {
	let p = Path::new("./tests/files/invalid/err4.txt");
	App::from_config_file(p);
}

#[test]
#[should_panic(expected = "No such file or directory")]
fn invalid_path() {
	let p = Path::new("./tests/files/invalid/err5.txt");
	App::from_config_file(p);
}

#[test]
#[should_panic(expected = "Cannot instantiate a ConflictResolver.")]
fn wrong_resolver_option() {
	let p = Path::new("./tests/files/invalid/err6.txt");
	App::from_config_file(p);
}
