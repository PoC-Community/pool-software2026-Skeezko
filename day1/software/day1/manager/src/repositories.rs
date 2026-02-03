use std::fs;
use crate::models::Task;


pub fn read()-> Vec<Task>{

    let list_str = fs::read_to_string("data/tasks.json").expect("File not found");

    let list = serde_json::from_str(&list_str).expect("Error reading JSON");

    return list;
}


pub fn write(new: Vec<Task>) {

    let tasks = serde_json::to_string_pretty(&new).expect("Error serializing");
    fs::write("data/tasks.json", tasks).expect("Failed to write");
}