mod file_manager;
use crate::file_manager::FileManager;

fn main() {
    match FileManager::list_files("./src") {
        Ok(files) => println!("\nFiles found : {:?}\n", files),
        Err(e) => eprintln!("\nError : {}\n", e),
    }
    match FileManager::read_file("./Cargo.toml") {
        Ok(content) => println!("Cargo.toml : {}\n", content),
        Err(e) => eprintln!("Error : {}\n", e),
    }
    match FileManager::write_file("./test.txt", "Hello, World!") {
        Ok(_) => println!("File successfully writen !\n"),
        Err(e) => eprintln!("Error : {}\n", e),
    }
    match FileManager::copy_file("./source.txt", "./destination.txt"){
        Ok(_) => println!("File copied successfully !\n"),
        Err(e) => eprintln!("Error : {}\n", e),
    }
}
