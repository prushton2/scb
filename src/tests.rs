#[cfg(test)]
mod tests {
	use crate::commands;
	use crate::file::{Config};
	use serial_test::serial;
	
	#[test]
	#[serial]

	//test the init and remove functions
	//if this test fails, all other tests will also fail
	fn test_init_remove() {
		
		commands::init();
		let mut exists = Config::load(commands::FILE_NAME);
		assert!(exists.is_ok());
	
		commands::remove();		
		exists = Config::load(commands::FILE_NAME);
		assert!(!exists.is_ok());
	}


	//the only array config option is files
	//just append, check, remove, check	
	#[test]
	#[serial]
	fn test_arr_config() {

		commands::init();
		let test_keys: [&str; 1] = ["files"];
		let test_array: &[String; 3] = &[String::from("file1"), String::from("file2"), String::from("string!")];

		for i in test_keys {

			commands::arr_config(i, "-a", test_array);
			let mut config: Config = Config::load(commands::FILE_NAME).unwrap();
			
			match i {
				"files" => {
					assert_eq!(config.files[0], test_array[0]);
					assert_eq!(config.files[1], test_array[1]);
					assert_eq!(config.files[2], test_array[2]);
				},
				&_ => {}
			}

			commands::arr_config(i, "-r", test_array);
			config = Config::load(commands::FILE_NAME).unwrap();
			
			match i {
				"files" => {
					assert_eq!(config.files.len(), 0);
				},
				&_ => {}
			}
		}

		commands::remove();
	}


	//there are 3 str config options, so we test all 3 of them with 3 different parameters
	#[test]
	#[serial]
	fn test_str_config() {

		commands::init();
		let test_keys: [&str; 3] = ["compiler", "header_dir", "outfile"];
		let test_array: [&str; 3] = ["test compiler", "gcc", "g++"];

		for i in test_array {
			for j in test_keys {
				
				commands::str_config(j, i);
				let config: Config = Config::load(commands::FILE_NAME).unwrap();
				
				//this feels like a  sin
				match j {
					"compiler" => assert_eq!(config.compiler, i),
					"header_dir" => assert_eq!(config.header_dir, i),
					"outfile" => assert_eq!(config.outfile, i),
					&_ => assert_eq!(1, 0)
				};
			}
		}
		
		commands::remove();

	}

	#[test]
	#[serial]
	fn test_build() {

		commands::init();
		commands::arr_config("files", "-a", &[String::from("main.c")]);

		let result = commands::build("");
		assert!(result.is_ok());
		
		commands::remove();

	}
}	