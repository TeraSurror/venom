use core::panic;
use std::{env, fs::File, io::Write, process::{self, Command}};

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

    // Create the project folder
    let create_directory = Command::new("mkdir").arg(project_name).output().expect("Failed to create the directory");
    println!("{}", String::from_utf8_lossy(&create_directory.stdout));
    if create_directory.status.success() {
        println!("Created project folder");
    } else {
        println!("Error in folder creation");
    }

    // Create the environment
    let create_environment = Command::new("virtualenv").arg(format!("{}/env", project_name)).output().expect("Could not create the environment");
    if create_environment.status.success() {
        println!("Created virtual environment!");
    } else {
        println!("Error in virtual environment creation");
    }

    // activate environment
    // I do not know how to do this
    // Halp me please

    // Create  .gitignore
    let mut file = match File::create(".gitignore") {
        Ok(f) => f, 
        Err(_) => panic!("Could not create gitignore file")
    };
    let data = "./env";
    file.write_all(data.as_bytes()).unwrap();

    // run git init
    let init_git = Command::new("git").arg("init").output().expect("Could not initialize git repository");
    if init_git.status.success() {
        println!("Created git repository!");
    } else {
        println!("Error in git repo creation");
    }

    // Create main.py
    let mut file = match File::create(format!("{}/main.py", project_name)) {
        Ok(f) => f, 
        Err(_) => panic!("Could not create main.py file")
    };
    let data = "def main():\n\tprint(\"Hello World!\")";
    file.write_all(data.as_bytes()).unwrap();


    // Create requirements.txt
    let mut file = match File::create(format!("{}/main.py", project_name)) {
        Ok(f) => f, 
        Err(_) => panic!("Could not create main.py file")
    };
    let data = "def main():\n\tprint(\"Hello World!\")";
    file.write_all(data.as_bytes()).unwrap();

    // Create requirements.txt
    match File::create("requirements.txt") {
        Ok(f) => {
            println!("Requirements file created");
            f
        }, 
        Err(_) => panic!("Could not create requirements file")
    };


    println!("Project Created successfully!");
    println!("Run - source env - to activate your virtual environment");
    println!("Happy hacking!");
}

fn log_error(error_msg: &str) {
    eprintln!("{}", error_msg);
    process::exit(1);
}
