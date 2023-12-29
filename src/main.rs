mod file;
mod commands;
mod tests;

fn main() {

	let mut args: file::Args = file::arguments();

	if args.argc < 1 {
		let successful = file::load_config(commands::FILE_NAME);
		if successful.is_ok() {
			let _ = commands::build("-r");
		} else {
			commands::init();
		}	
		return;
	}

	match args.argv[0].as_str() {
		"init" => commands::init(),
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


