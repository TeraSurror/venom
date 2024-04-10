use core::panic;
use std::env;

use venom::commands;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Incorrect number of arguments");
    }

    let command = &args[1];

    match command.as_str() {
        "init" => {
            // Validate the command
            if args.len() < 3 {
                panic!("Project name is not mentioned");
            }
            
            // initialize the project
            let project_name = &args[2];
            commands::init::run_init_command(project_name);
        },
        "install" => {
            // Validate the command
            if args.len() < 3 {
                panic!("Project name is not mentioned");
            }

            // Get the lib to install
            let lib_name = &args[2];
            commands::install::run_install(lib_name);
        }
        _ => panic!("Kya likha hai bhai??")
    }
}
