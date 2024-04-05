use std::{env, process::{self, Command}};

const ENV_PATH: &str  = "/home/harshshealar22/Projects/python-env";

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        log_error("Incorrect number of arguments");
    }

    let command = &args[1];

    if command == "init" {
        if args.len() < 3 {
            log_error("project name not mentioned");
        }

        let project_name = &args[2];

        // Create the environment
   
        // Activate the environment
        
        // Create the project folder
        let create_directory = Command::new("mkdir").args(&[project_name]).output().expect("Failed to create the directory");
        println!("{}", String::from_utf8_lossy(&create_directory.stdout));

        println!("Project Created successfully!");
    }
}

fn log_error(error_msg: &str) {
    eprintln!("{}", error_msg);
    process::exit(1);
}
