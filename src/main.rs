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
        init_logic(&args[2]);
    }
}

fn init_logic(project_name: &str) {

    let env_name = format!("{}/{}", ENV_PATH, project_name);
    let env_bin_path = format!("{}/{}/bin/activate", ENV_PATH, project_name);

    // Create the environment
    let create_environment = Command::new("virtualenv").arg(env_name).output().expect("Could not create the environment");

    if create_environment.status.success() {
        println!("Created virtual environment!");
    } else {
        println!("Error in virtual environment creation");
    }

    // Create the project folder
    let create_directory = Command::new("mkdir").arg(project_name).output().expect("Failed to create the directory");
    println!("{}", String::from_utf8_lossy(&create_directory.stdout));

    if create_directory.status.success() {
        println!("Created project folder");
    } else {
        println!("Error in folder creation");
    }

    println!("Project Created successfully!");
    println!("Run - source {} - to activate your virtual environment", env_bin_path);
    println!("Happy hacking!")

}

fn log_error(error_msg: &str) {
    eprintln!("{}", error_msg);
    process::exit(1);
}
