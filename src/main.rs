use std::env;
use serde::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize, Deserialize)]
struct Config {
	compiler: String,
	header_dir: String,
	outfile: String,
	// files: &'a [&'a str],
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


// static FILE_NAME: &str = "SCB";

fn load_config(file_name: &str) {
	let config: Config = Config {
		compiler: "comp".to_string(),
		header_dir: "headir".to_string(),
		outfile: "of".to_string(),
		// files: &["f1", "f2", "f3"],
	};


    let j = serde_json::to_string(&config);
	
	if j.is_ok() {
		println!("{:?}", j.ok());
	}

	// if serialized.is_ok() {
	// println!("{}", j);	// } else {
		// println!("{:#?}", serialized.err());
	// }



	// return config;
}


// fn init() {}
// fn build() {}
// fn help() {}

fn str_config(key: &str, value: &str) {
	println!("{}: {}", key, value);
}

fn arr_config(key: &str, action: &str, value: &[&str]) {
	// for i in value {
	// 	println!("{}", i);
	// }

	// let config: Config = 
	load_config("aaaaaaa");
	// println!("{}\n{}\n{}", config.compiler, config.header_dir, config.outfile);
	// println!("{}\n{}\n{}", config.files[0], config.files[1], config.files[2]);
}

fn main() {

	let args: Args = arguments();

	if args.argc < 1 {
		return;
	}

	match args.argv[0].as_str() {
		"compiler" => str_config("compiler", args.argv[1].as_str()),
		"file" => arr_config("compiler", "-a", &["1", "2", "3"]),
		_ => println!("NAAAAAAAHHHH"),
	}

}
