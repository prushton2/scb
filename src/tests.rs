#[cfg(test)]
mod tests {
	use crate::commands;
	use crate::file;
	use serial_test::serial;
	
	#[test]
	#[serial]

	//test the init and remove functions
	//if this test fails, all other tests will also fail
	fn test_init_remove() {
		
		commands::init();
		let mut exists = file::load_config(commands::FILE_NAME);
		assert!(exists.is_ok());
	
		commands::remove();		
		exists = file::load_config(commands::FILE_NAME);
		assert!(!exists.is_ok());
	}


	//the only array config option is files, so no need for a loop here
	//just append, check, remove, check	
	#[test]
	#[serial]
	fn test_arr_config() {

		commands::init();
		let files: &[String; 2] = &[String::from("file1"), String::from("file2")];

		commands::arr_config("files", "-a", files);
		let mut config: file::Config = file::load_config(commands::FILE_NAME).unwrap();

		assert_eq!(config.files[0], files[0]);
		assert_eq!(config.files[1], files[1]);

		commands::arr_config("files", "-r", files);
		config = file::load_config(commands::FILE_NAME).unwrap();

		assert_eq!(config.files.len(), 0);

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
				let config: file::Config = file::load_config(commands::FILE_NAME).unwrap();
				
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
}	