Sure! Here's the entire `README.md` content in a **single copy-pasteable block** — just copy and paste this into your `README.md` file:

---

````markdown
# 📝 Native Rust CLI To-Do List Manager

A simple command-line To-Do application built using **pure Rust** — no external crates or frameworks. This project is designed to help you understand Rust's standard library, ownership model, file I/O, enums, and error handling.

---

## 🚀 Features

- 📌 Add new tasks
- 📋 List all tasks
- ✅ Mark tasks as done
- ❌ Delete tasks
- 💾 Persistent storage in a local file (`todos.txt`)
- 🧱 Built 100% with Rust standard library

---

## 🛠️ Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/todo-cli.git
cd todo-cli
````

### 2. Build the Project

```bash
cargo build
```

### 3. Run the Application

You can use the following commands:

#### ➕ Add a Task

```bash
cargo run -- add "Buy groceries"
```

#### 📋 List All Tasks

```bash
cargo run -- list
```

#### ✅ Mark a Task as Done

```bash
cargo run -- done 1
```

#### ❌ Delete a Task

```bash
cargo run -- delete 1
```

---

## 💾 Data Format

All tasks are saved in `todos.txt` in the following format:

```
1|Buy groceries|false
2|Learn Rust|true
```

Each line represents a task with:

* `id | title | status`

---

## 🧱 Project Structure

```
src/
├── main.rs         # Entry point and argument handling
├── todo.rs         # TodoItem struct and logic
├── storage.rs      # File read/write logic
└── commands.rs     # CLI command parsing and execution
```

---

## 💡 Example Usage

```bash
$ cargo run -- add "Write README"
Added: Write README

$ cargo run -- list
[ ] 1 - Write README

$ cargo run -- done 1
Marked done: Write README

$ cargo run -- list
[x] 1 - Write README
```

---

## 📚 Built With

* 🦀 [Rust](https://www.rust-lang.org/) (Standard Library Only)
* 🧠 Human Brain
* ⌨️ Terminal & Text Editor

---

## 📜 License

This project is licensed under the MIT License.
Feel free to fork, modify, and use it for learning or productivity!

---

## 👨‍💻 Author

**Your Name**
[GitHub Profile](https://github.com/piyush-itis)

```

---

```
