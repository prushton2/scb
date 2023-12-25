use std::env;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::io::Write;
use std::process::{Command};
use execute::Execute;


#[derive(Serialize, Deserialize)]
struct Config {
	compiler: String,
	header_dir: String,
	outfile: String,
	files: Vec<String>,
}

struct Args {
	argc: u32,
	argv: Vec<String>
}

fn arguments<'a>() -> Args {
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


static FILE_NAME: &str = "scb";

fn load_config(file_name: &str) -> Result<Config, &str> {
	let file_contents = fs::read_to_string(file_name);


	if file_contents.is_err() {
		return Err("File could not be read")
	}

	let config: Config = serde_json::from_str(&file_contents.unwrap()).unwrap();

	return Ok(config)
}


fn write_config(file_name: &str, config: Config) -> Result<&str, &str> {
	
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

fn init() {
	let config: Config = Config {
		compiler: "gcc".to_string(),
		header_dir: "../".to_string(),
		outfile: "main".to_string(),
		files: vec![]
	};

	let _ = write_config(FILE_NAME, config);
}

fn build(args: &str) -> Result<&str, &str> {
	let config_result = load_config(FILE_NAME);
	
	if config_result.is_err() {
		return Err("Error loading config");
	}

	let config = config_result.unwrap();
	
	let mut command = Command::new(config.compiler);

	command.arg("-I");
	command.arg(config.header_dir.clone());
	for i in config.files {
		command.arg(i);
	}
	command.arg("-o");
	command.arg(config.outfile.clone());
	
	let output_result = command.execute_output();

	if output_result.is_err() {
		return Err("Build failed");
	}

	if args == "-r" {
		let run_result = Command::new(format!("./{}", config.outfile)).execute_output();
	
		if run_result.is_err() {
			return Err("Run failed, build succeeded");
		}
	}

	return Ok("Ran successfully");
	
}
// fn help() {}

fn str_config(key: &str, value: &str) {
	let result: Result<Config, &str> = load_config(FILE_NAME);
	if result.is_err() {
		println!("Error loading config");
		return;
	}

	let mut config: Config = result.unwrap();

	let mut object: String = String::from("");

	match key { //make sure nothing dumb can be done (like referencing a nonexistent key)
		"compiler" => object = config.compiler.clone(),
		"header_dir" => object = config.header_dir.clone(),
		"outfile" => object = config.outfile.clone(),
		&_ => println!("Invalid Key")
	}

	if value == "" {
		println!("{}: {}", key, object);
	} else {
		object = value.to_string();
		println!("Set {} to {}", key, value);
	}
	

	match key {
		"compiler" => config.compiler = object.clone(),
		"header_dir" => config.header_dir = object.clone(),
		"outfile" => config.outfile = object.clone(),
		&_ => println!("You shouldnt be here")
	}


	let _ = write_config(FILE_NAME, config);
}

fn arr_config(key: &str, action: &str, value: &[String]) {
	let result: Result<Config, &str> = load_config(FILE_NAME);
	if result.is_err() {
		println!("Error loading config");
		return;
	}

	let mut config: Config = result.unwrap();

	let mut object: Vec<String> = vec![];

	match key {
		"files" => object = config.files.clone(),
		&_ => println!("Invalid Key")
	}


	match action {
		"-ls" => {
			println!("Tracked Files:");
			for i in &object {
				print!("{}, ", i);
			}
			println!("");
		},
		"-r" => {
			for i in value {
				let _ = object.retain(|e| e != i);
			}
		},
		"-a" => {
			for i in value {
				object.push(i.clone());
			}
		},
		&_ => {
			println!("Unrecognized flag {}", action);
		}
	}

	
	match key {
		"files" => config.files = object.clone(),
		&_ => println!("You shouldnt be here")
	}


	let _ = write_config(FILE_NAME, config);
}

fn main() {

	let mut args: Args = arguments();

	if args.argc < 1 {
		let successful = load_config(FILE_NAME);
		if successful.is_ok() {
			let _ = build("-r");
		} else {
			init();
		}	
		return;
	}

	match args.argv[0].as_str() {
		"init" => init(),
		"build" => {
			if args.argc < 2 {
				args.argv.push(String::from(""));
			}
			let _ = build(args.argv[1].as_str());
		}
		"compiler" | "header_dir" | "outfile" => {
			if args.argc < 2 {
				args.argv.push(String::from(""));
				args.argv.push(String::from(""));
			}
			
			str_config(args.argv[0].as_str(), &args.argv[1].as_str());
		},
		"file" => {
			
			if args.argc < 3 {
				arr_config("files", "-ls", &[String::from("")]);
				return;
			}

			arr_config("files", args.argv[1].as_str(), &args.argv[2..]);
		},
		_ => println!("Incorect Function"),
	}

}
