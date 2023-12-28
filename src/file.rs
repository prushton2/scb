use serde_json;
use serde::{Serialize, Deserialize};
use std::fs;
use std::env;
use std::io::Write;


#[derive(Serialize, Deserialize)]
pub struct Config {
	pub compiler: String,
	pub header_dir: String,
	pub outfile: String,
	pub files: Vec<String>,
}

pub struct Args {
	pub argc: u32,
	pub argv: Vec<String>
}


pub fn arguments<'a>() -> Args {
	let mut args: Args = Args {
		argc: 0,
		argv: vec![]
	};


	let mut flag: bool = true;

	while flag {
		let arg = env::args().nth((args.argc+1) as usize);
		if arg.is_some() {
			args.argc += 1;
			args.argv.push(arg.unwrap());
		} else {
			flag = !flag;
		}
	}


	return args;
}



pub fn load_config(file_name: &str) -> Result<Config, &str> {
	let file_contents = fs::read_to_string(file_name);


	if file_contents.is_err() {
		return Err("File could not be read")
	}

	let config: Config = serde_json::from_str(&file_contents.unwrap()).unwrap();

	return Ok(config)
}


pub fn write_config(file_name: &str, config: Config) -> Result<&str, &str> {
	
	let file_result = fs::File::create(file_name);
	let serialized = serde_json::to_string(&config);

	if serialized.is_err() {
		return Err("Serialization Failed")
	}

	if file_result.is_err() {
		return Err("Error opening file")
	}

	let mut file = file_result.unwrap();

	let _ = file.write_all(serialized.unwrap().as_bytes());
	return Ok("File Written")
}

