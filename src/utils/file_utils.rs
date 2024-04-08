use std::fs::File;
use std::io::Write;

pub struct FileUtils {}

impl FileUtils {
   pub fn create_empty_file(file_path: String) {
       File::create(&file_path).expect("Failed to create file");
   } 

   pub fn create_file_with_content(file_path: String, content: &str) {
       let mut file = File::create(&file_path).expect("Failed to create file");
       file.write_all(content.as_bytes()).expect("Failed to write to file");
   }
}
