use crate::repositories;
use crate::views;
use crate::models::Task;


pub fn list_tasks() {

    views::display_list(repositories::read());
}


pub fn add_task() {

    let task_description = views::ask_description();
    let mut tasks = repositories::read();
    let task_id = (tasks.len() + 1).try_into().unwrap();

    let new_task = Task {
        id: task_id,
        description: task_description,
        completed: false,
    };

    tasks.push(new_task);

    repositories::write(tasks);
    views::add_success();
}


pub fn edit_task() {

    let task_to_edit = views::ask_task_to_edit();
    let task_description = views::ask_description();
    let mut tasks = repositories::read();
    let task_count = tasks.len().try_into().unwrap();

    if task_to_edit == 0 || task_to_edit > task_count {
        views::not_found();
        return;
    }

    for task in tasks.iter_mut(){
        if task.id == task_to_edit {
            task.description = task_description;
            break;
        }
    }

    repositories::write(tasks);
    views::edit_success();
}


pub fn mark_task() {

    let task_to_mark = views::ask_task_to_mark();
    let mut tasks = repositories::read();
    let task_count = tasks.len().try_into().unwrap();

    if task_to_mark == 0 || task_to_mark > task_count {
        views::not_found();
        return;
    }

    for task in tasks.iter_mut(){
        if task.id == task_to_mark {
            task.completed = true;
            break;
        }
    }

    repositories::write(tasks);
    views::mark_success();
}


pub fn delete_task() {

    let task_to_delete = views::ask_task_to_delete();
    let mut tasks = repositories::read();
    let task_count = tasks.len().try_into().unwrap();

    if task_to_delete == 0 || task_to_delete > task_count {
        views::not_found();
        return;
    }

    tasks.retain(|task| task.id != task_to_delete);

    for task in tasks.iter_mut(){
        if task.id > task_to_delete {
            task.id = task.id - 1;
        }
    }

    repositories::write(tasks);
    views::delete_success();
}