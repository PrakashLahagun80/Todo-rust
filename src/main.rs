use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {

    fn new(id: u32, title: String, completed: bool) -> Todo {
        Todo { id, title, completed }
    }
    fn mark_completed(&mut self) {
        self.completed = true
    }
}
fn main() {

   let mut todos = load_todos().unwrap_or_else(|_| Vec::new()); 
   
   loop {
        println!("Todo App");
        println!("1. Add a task");
        println!("2. Mark a task as completed");
        println!("3. View all tasks");
        println!("4. Delete a task");
        println!("5. Exit");

        let mut choice = String::new();
        if let Err(e) = io::stdin().read_line(&mut choice) {
            eprint!("Error reading input: {}", e);
            continue;
        }

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };

    match choice {
        1 => add_task(&mut todos),
        2 => mark_completed(&mut todos),
        3 => view_all_tasks(&todos),
        4 => delete_task(&mut todos),
        5 => break,
        _ => println!("Invalid choice"),
    }

    if let Err(e) = save_todos(&todos) {
        eprint!("Error saving todos: {}", e);
    }
 }
}

fn add_task(todos: &mut Vec<Todo>) {
    let id = todos.len() as u32 + 1;
    let mut title = String::new();
    println!("Enter task title:");
    if let Err(e) = io::stdin().read_line(&mut title) {
        eprint!("Error reading input: {}", e);
        return;
    }
    let title = title.trim().to_string();
    let todo = Todo::new(id, title, false);
    todos.push(todo);
    println!("Task added successfully");
}

fn view_all_tasks(todos: &Vec<Todo>) {
    println!("All tasks:");
    for todo in todos {
        println!("{} - {}", todo.id, todo.title);
    }
}

fn delete_task(todos: &mut Vec<Todo>) {
    let mut id = String::new();
    println!("Enter task id to delete:");
    if let Err(e) = io::stdin().read_line(&mut id) {
        eprint!("Error reading input: {}", e);
        return;
    }
    let id: u32 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid task id");
            return;
        }
    };
    let index = todos.iter().position(|t| t.id == id);
    
    match index {
        Some(i) =>{ let _ =  todos.remove(i); },
        None => println!("Task not found"),
    }
}

fn mark_completed(todos: &mut Vec<Todo>) {
    let mut id = String::new();
    println!("Enter the ID of the task to mark as completed:");
    if let Err(e) = io::stdin().read_line(&mut id) {
        eprintln!("Error reading input: {}", e);
        return;
    }
    let id: u32 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };
    
 if let Some(index) = todos.iter().position(|t| t.id == id) {
        todos[index].mark_completed();
        todos.remove(index);
        println!("Task marked as completed");
    } else {
        println!("Task not found");
    }
}

fn load_todos() -> Result<Vec<Todo>, io::Error> {
    let data = fs::read_to_string("todo.json")?;
    let todos = serde_json::from_str(&data)?;
    Ok(todos)
}

fn save_todos(todos: &Vec<Todo>) -> Result<(), io::Error> {
    let data = serde_json::to_string(todos)?;
    fs::write("todo.json", data)?;
    Ok(())
}