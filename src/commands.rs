use crate::file::{Config};
use std::process::{Command};
use execute::Execute;
use std::time::{SystemTime, UNIX_EPOCH};

pub const FILE_NAME: &str = "scb"; //not great practice to keep it in here


pub fn init() {
	let config: Config = Config {
		compiler: "gcc".to_string(),
		header_dir: "../".to_string(),
		outfile: "main".to_string(),
		files: vec![],
	    start_time: 0
    };

	let _ = config.write(FILE_NAME);
}

pub fn build(args: &str, return_output: bool) -> Result<String, &str> {
	let config_result = Config::load(FILE_NAME);
	
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
		let mut run_result = Command::new(format!("./{}", config.outfile));
	
        if return_output {
            let output = run_result.output().unwrap();
            let out_string = String::from_utf8_lossy(&output.stdout);

            return Ok(out_string.to_string());
        } else {
            
            let run_output = run_result.execute_output();
            
            if run_output.is_err() {
                return Err("Run failed, build succeeded");
            }
		}
	}

   // if return_output {
//
  //      let bytes: Vec<u8> = output_result.unwrap().stdout;
  //      let output: String = String::from_utf8(bytes.clone()).unwrap();
  //      
  //      println!("{}", output);
//
//        return Ok(output.clone());
//    }

	return Ok("Ran successfully".to_string());
	
}

pub fn help() {
	println!("SCB Help

scb
	initializes the project if not initialized
	if initialized, it compiles and runs the code

init
	initializes the build tool by creating the file scb. Add this to your gitignore

remove
	removes the scb file

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
	let result: Result<Config, &str> = Config::load(FILE_NAME);
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


	let _ = config.write(FILE_NAME);
}

pub fn arr_config(key: &str, action: &str, value: &[String]) {
	let result: Result<Config, &str> = Config::load(FILE_NAME);
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


	let _ = config.write(FILE_NAME);
}


pub fn remove() {
	let result = Config::delete(FILE_NAME);

	if result.is_ok() {
		println!("Removed SCB");
	} else {
		println!("Error removing SCB:\n {:?}", result.err());
	}
}

pub fn speedrun_start<'a>() -> Result<&'a str, &'a str>{
    let mut config = Config::load(FILE_NAME)?;
 
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
 
    config.start_time = since_the_epoch.as_millis();

    let _ = config.write(FILE_NAME);

    return Ok("Ok");
}

pub fn speedrun_build() {
    let mut config = Config::load(FILE_NAME).unwrap();
    
    let expected_output_full = Config::speedrun_read_expected_output("./scb_expected_output.txt").unwrap(); //[..expected_output.len()-1];
    let expected_output = &expected_output_full[..expected_output_full.len()-1];

    let output = build("-r", true).unwrap();
    
    println!("Output: \n{:?}\n\nExpected Output:\n{:?}", output, expected_output);
    
    if output == expected_output {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        
        println!("\nComplete! Time stopped at {}m {}s", 
                
                (since_the_epoch.as_millis()- config.start_time)/1000/60,
                (since_the_epoch.as_millis() as f64 - config.start_time as f64)/1000.000%60.0

            );
        
        config.start_time = 0;
        let _ = config.write(FILE_NAME);

    }
}
