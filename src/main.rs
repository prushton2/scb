use std::env;

struct Config<'a> {
	compiler: &'a str,
	header_dir: &'a str,
	outfile: &'a str,
	files: &'a [&'a str],
}

static FILE_NAME: &str = "SCB";


fn load_config(file_name: &str) -> Config {
	let config: Config = Config {
		compiler: "comp",
		header_dir: "headir",
		outfile: "of",
		files: &["f1", "f2", "f3"],
	};

	return config;
}


// fn init() {}
// fn build() {}
// fn help() {}

fn str_config(key: &str, value: &str) {
	println!("{}: {}", key, value);
}

fn arr_config(key: &str, action: &str, value: &[&str]) {
	for i in value {
		println!("{}", i);
	}

	let config: Config = load_config("aaaaaaa");
	println!("{}\n{}\n{}", config.compiler, config.header_dir, config.outfile);
	println!("{}\n{}\n{}", config.files[0], config.files[1], config.files[2]);
}

fn main() {
	let arg0 = env::args().nth(1).unwrap();
	let arg1 = env::args().nth(2).unwrap();


	match arg0.as_str() {
		"compiler" => str_config("compiler", arg1.as_str()),
		"files" => arr_config("compiler", "-a", &["1", "2", "3"]),
		_ => println!("NAAAAAAAHHHH"),
	}

}
