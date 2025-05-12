use crate::todo::TodoItem;
use crate::storage::{load_todos, save_todos};

pub fn handle_command(args: Vec<String>) {
    let mut todos = load_todos();

    match args.get(1).map(|s| s.as_str()) {
        Some("add") => {
            if let Some(title) = args.get(2) {
                let id = (todos.last().map(|t| t.id).unwrap_or(0)) + 1;
                todos.push(TodoItem::new(id, title.clone()));
                println!("Added: {}", title);
            } else {
                println!("Usage: todo-cli add <task>");
            }
        }

        Some("list") => {
            for todo in &todos {
                let status = if todo.done { "[x]" } else { "[ ]" };
                println!("{} {} - {}", status, todo.id, todo.title);
            }
        }

        Some("done") => {
            if let Some(id_str) = args.get(2) {
                if let Ok(id) = id_str.parse::<u32>() {
                    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                        todo.done = true;
                        println!("Marked done: {}", todo.title);
                    } else {
                        println!("Task not found.");
                    }
                }
            }
        }

        Some("delete") => {
            if let Some(id_str) = args.get(2) {
                if let Ok(id) = id_str.parse::<u32>() {
                    todos.retain(|t| t.id != id);
                    println!("Deleted task {}", id);
                }
            }
        }

        _ => {
            println!("Commands: add, list, done, delete");
        }
    }

    save_todos(&todos);
}
