use std::io;
use crate::controllers;


pub fn router(){

    println!("\nWelcome to your Task Manager!");

    let mut menu = true;

    loop {
        if menu {println!("\nWhat do you want to do?\n1 - List all tasks\n2 - Add a task\n3 - Edit a task\n4 - Mark a task\n5 - Delete a task\n6 - Leave\n");}

        let mut rep = String::new();
        io::stdin().read_line(&mut rep).expect("Failed to read line");

        match rep.trim() {
            "1" => {
                controllers::list_tasks();
                menu = true;
            }
            "2" => {
                controllers::add_task();
                menu = true;
            }
            "3" => {
                controllers::list_tasks();
                controllers::edit_task();
                menu = true;
            }
            "4" => {
                controllers::list_tasks();
                controllers::mark_task();
                menu = true;
            }
            "5" => {
                controllers::list_tasks();
                controllers::delete_task();
                menu = true;
            }
            "6" => break,
            _ => {
                println!("\nType 1, 2, 3, 4, 5 or 6.\n");
                menu = false;
            }
        }
    }
    println!("\nSee you!\n")
}