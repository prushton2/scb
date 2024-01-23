mod commands;
mod tests;
mod file;

use crate::file::{Config, Args};


fn main() {

	let mut args: Args = Args::get();

	if args.argc < 1 {
		let successful = Config::load(commands::FILE_NAME);
		if successful.is_ok() {
			let _ = commands::build("-r");
		} else {
			commands::init();
		}	
		return;
	}

	match args.argv[0].as_str() {
		"init" => commands::init(),
		"remove" => commands::remove(),
		"build" => {
			if args.argc < 2 {
				args.argv.push(String::from(""));
			}
			let _ = commands::build(args.argv[1].as_str());
		}
		"compiler" | "header_dir" | "outfile" => {
			if args.argc < 2 {
				args.argv.push(String::from(""));
				args.argv.push(String::from(""));
			}
			
			commands::str_config(args.argv[0].as_str(), &args.argv[1].as_str());
		},
		"file" => {
			
			if args.argc < 3 {
				commands::arr_config("files", "-ls", &[String::from("")]);
				return;
			}

			commands::arr_config("files", args.argv[1].as_str(), &args.argv[2..]);
		},
		"-h" | "--help" => {
			commands::help();
		}
		_ => println!("Incorect Function"),
	}

}


