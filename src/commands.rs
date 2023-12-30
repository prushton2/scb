use crate::file;
use std::process::{Command};
use execute::Execute;

pub const FILE_NAME: &str = "scb"; //not great practice to keep it in here


pub fn init() {
	let config: file::Config = file::Config {
		compiler: "gcc".to_string(),
		header_dir: "../".to_string(),
		outfile: "main".to_string(),
		files: vec![]
	};

	let _ = file::write_config(FILE_NAME, config);
}

pub fn build(args: &str) -> Result<&str, &str> {
	let config_result = file::load_config(FILE_NAME);
	
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

pub fn help() {
	println!("SCB Help

scb
	initializes the project if not initialized
	if initialized, it compiles and runs the code

init
	initializes the build tool by creating the file scb. Add this to your gitignore

file
	Manages files the build tool should keep track of
	Arguments
	-a   [files]  Takes a list of files and tracks them
	-r   [files]  Takes a list of files and untracks them
	-ls           Lists tracked files

header
	Displays the current header directory
	Arguments
	-s   [dir]  Sets the directory as the header directory

out
	Displays the outfile
	Arguments
	-s   [file]  Sets the file as the outfile
	
compiler
	Displays the current compiler
	Arguments
	-s   [compiler]  Updates the compiler

build
	Builds the project
	Arguments
	-r   Runs the project after building
	");
}

pub fn str_config(key: &str, value: &str) {
	let result: Result<file::Config, &str> = file::load_config(FILE_NAME);
	if result.is_err() {
		println!("Error loading config");
		return;
	}

	let mut config: file::Config = result.unwrap();

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


	let _ = file::write_config(FILE_NAME, config);
}

pub fn arr_config(key: &str, action: &str, value: &[String]) {
	let result: Result<file::Config, &str> = file::load_config(FILE_NAME);
	if result.is_err() {
		println!("Error loading config");
		return;
	}

	let mut config: file::Config = result.unwrap();

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


	let _ = file::write_config(FILE_NAME, config);
}


pub fn remove() {
	let result = file::delete_file(FILE_NAME);

	if result.is_ok() {
		println!("Removed SCB");
	} else {
		println!("Error removing SCB:\n {:?}", result.err());
	}
}