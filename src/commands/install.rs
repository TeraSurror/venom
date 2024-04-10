use std::{fs::{File, OpenOptions}, io::{ErrorKind, Write}, process::{Command, Stdio}};

pub fn run_install(library_name: &[String]) {

    // Run the install script
    println!("[1] Installing Packages...");
    if let Err(err) = Command::new("pip").arg("install").args(library_name).output() {
        panic!("Could not install libs: {}", err);
    }

    // Update the requirement.txt file
    println!("[2] Updating requirements...");
    let output = Command::new("pip")
        .args(&["freeze", "--local"])
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to execute command");

    let mut file = match File::open("requirements.txt") {
        Ok(_) => {
            OpenOptions::new().read(true).write(true).open("requirements.txt").expect("Failed to open file")
        }
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            File::create("requirements.txt").expect("Failed to create file")
        }
        Err(err) => {
            panic!("Failed to open file: {}", err)
        }
    };

    // Append the output to the file
    file.write_all(&output.stdout).expect("Failed to write to file");

    println!("Libs installed successfully!");
}
