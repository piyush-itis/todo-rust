mod todo;
mod storage;
mod commands;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    commands::handle_command(args);
}
