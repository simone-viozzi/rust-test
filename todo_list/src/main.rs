use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

const TODO_FILE: &str = "todo_list.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        TodoList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let id = self.tasks.len() + 1;
        self.tasks.push(Task { id, description });
        println!("Task added: {}", id);
    }

    fn remove_task(&mut self, id: usize) {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(pos);
            println!("Task removed: {}", id);
        } else {
            println!("Task not found: {}", id);
        }
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available!");
        } else {
            for task in &self.tasks {
                println!("{}: {}", task.id, task.description);
            }
        }
    }

    fn save_to_file(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Failed to serialize tasks.");
        fs::write(TODO_FILE, json).expect("Failed to write tasks to file.");
    }

    fn load_from_file() -> Self {
        if Path::new(TODO_FILE).exists() {
            let json = fs::read_to_string(TODO_FILE).expect("Failed to read tasks from file.");
            serde_json::from_str(&json).expect("Failed to parse tasks from file.")
        } else {
            TodoList::new()
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task to the todo list
    Add { description: String },
    /// Remove a task from the todo list
    Remove { id: usize },
    /// List all tasks in the todo list
    List,
}

fn main() {
    let args = Cli::parse();
    let mut todo_list = TodoList::load_from_file();

    match args.command {
        Commands::Add { description } => {
            todo_list.add_task(description);
        }
        Commands::Remove { id } => {
            todo_list.remove_task(id);
        }
        Commands::List => {
            todo_list.list_tasks();
        }
    }

    todo_list.save_to_file();
}
