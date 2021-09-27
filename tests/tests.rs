use synkronizer::{App, Path};

#[test]
fn file_is_correct() {
	let base_path = "./tests/files/correct/config{}.txt";
	let path = base_path.replace("{}", "");
	let path = Path::new(&path);
	let base_config = App::from_config_file(path);

	for i in 1..=4 {
		let file_name = base_path.replace("{}", &i.to_string());
		let path = Path::new(&file_name);
		let x = App::from_config_file(path);
		assert_eq!(base_config.home, x.home);
		assert_eq!(base_config.config, x.config);
	}
}

#[test]
fn parses_whitespace_path() {
	let p = Path::new("./tests/files/correct/config5.txt");
	let x = App::from_config_file(p);

	assert_eq!(x.config, x.home);
	assert!(x.home.is_dir());
	assert!(x.config.is_dir());
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
