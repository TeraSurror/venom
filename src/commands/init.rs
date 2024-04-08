use core::panic;
use std::{fs::{self}, process::Command};

use crate::utils::file_utils::FileUtils;

pub fn run_init_command(project_name: &str) {

    // Create the project folder
    if let Err(err) = fs::create_dir(project_name) {
        panic!("Failed to create the directory: {}", err);
    }

    // Create the virtual environment
    let env_dir = format!("{}/env", project_name);
    if let Err(err) = Command::new("virtualenv").arg(env_dir).output() {
        panic!("Could not create the environment: {}", err);
    } 

    // activate environment
    // I do not know how to do this
    // Halp me please

    // Create  .gitignore
    let gitignore_dir = format!("{}/.gitignore", project_name);
    let gitignore_content = "./env";
    FileUtils::create_file_with_content(gitignore_dir, gitignore_content);

    // Create git repo
    if let Err(err) = Command::new("git")
        .arg("init")
        .arg(format!("{}", project_name))
        .output() {
            panic!("Could not initialize git repository: {}", err);
        };

    // Create main.py
    let main_file_path = format!("{}/main.py", project_name);
    let main_file_content = "def main():\n\tprint(\"Hello World!\")";
    FileUtils::create_file_with_content(main_file_path, main_file_content);

    // Create requirements.txt
    let req_file_path = format!("{}/requirements.txt", project_name);
    FileUtils::create_empty_file(req_file_path);

    println!("");
    println!("Project Created successfully!");
    println!("");
    println!("Begin by typing the following commands:");
    println!("cd {}", project_name);
    println!("source env");
    println!("");
    println!("Happy hacking!");
    println!("");

}
