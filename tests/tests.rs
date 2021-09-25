use synKronizer::{App, Path};

// #[test]
// fn this_test_fails() {
//     panic!();
// }

#[test]
fn file_is_correct() {
	let base_path = "./tests/files/correct/config{}.txt";
	let base_config = App::from_config_file(Path::new(&base_path.replace("{}", "")));

	for i in 1..=4 {
		let file_name = base_path.replace("{}", &i.to_string());
		let x = App::from_config_file(Path::new(&file_name));
		assert_eq!(base_config.home, x.home);
		assert_eq!(base_config.config, x.config);
	}
}
