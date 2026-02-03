use std::io;
use crate::models::Task;


pub fn display_list(list: Vec<Task>) {

    println!("\nHere are your tasks:");

    for task in list {
        if task.completed == false {println!("-- {} -- [ ] {}", task.id, task.description)} else {println!("-- {} -- [X] {}", task.id, task.description)};
    }
}


pub fn ask_description()-> std::string::String{

    println!("\nPlease enter a description:\n");

    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");

    return description.trim().to_string();
}


pub fn ask_task_to_edit() -> u32{

    println!("\nWhich one do you want to edit?\n");

    loop {
        let mut task_to_edit = String::new();
        io::stdin().read_line(&mut task_to_edit).expect("Failed to read line");
        
        match task_to_edit.trim().parse::<u32>() {
            Ok(id) => return id,
            Err(_) => println!("\nType the id of the task you want to edit.\n")
        }
    }
}


pub fn ask_task_to_mark() -> u32{

    println!("\nWhich one do you want to mark?\n");

    loop {
        let mut task_to_mark = String::new();
        io::stdin().read_line(&mut task_to_mark).expect("Failed to read line");
        
        match task_to_mark.trim().parse::<u32>() {
            Ok(id) => return id,
            Err(_) => println!("\nType the id of the task you want to mark.\n")
        }
    }
}


pub fn ask_task_to_delete() -> u32{

    println!("\nWhich one do you want to delete?\n");

    loop {
        let mut task_to_delete = String::new();
        io::stdin().read_line(&mut task_to_delete).expect("Failed to read line");
        
        match task_to_delete.trim().parse::<u32>() {
            Ok(id) => return id,
            Err(_) => println!("\nType the id of the task you want to delete.\n")
        }
    }
}


pub fn add_success() {
    println!("\nTask added !");
}

pub fn edit_success() {
    println!("\nTask edited !");
}

pub fn mark_success() {
    println!("\nTask marked !");
}

pub fn delete_success() {
    println!("\nTask deleted !");
}

pub fn not_found() {
    println!("\nTask not found !")
}