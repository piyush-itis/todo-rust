use std::fs::{File};
use std::io::{BufRead, BufReader, Write};
use crate::todo::TodoItem;

const FILE_NAME: &str = "todos.txt";

pub fn load_todos() -> Vec<TodoItem> {
    let file = File::open(FILE_NAME).unwrap_or_else(|_| File::create(FILE_NAME).unwrap());
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| TodoItem::from_line(&line.unwrap()))
        .collect()
}

pub fn save_todos(todos: &Vec<TodoItem>) {
    let mut file = File::create(FILE_NAME).expect("Failed to open file");
    for todo in todos {
        writeln!(file, "{}", todo.format()).expect("Failed to write");
    }
}
