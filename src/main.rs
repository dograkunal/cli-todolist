mod tasks;

use std::io;
use std::io::Write;
use tasks::{add_task, mark_task_as_completed, remove_task, view_tasks, load_tasks};

fn main() {
    // load tasks from file
    let mut tasks = load_tasks();

    loop {
        println!("\n");
        println!("--------------------------------");
        println!("To-Do List");
        println!("--------------------------------");
        println!("1. Add a task");
        println!("2. View tasks");
        println!("3. Mark task as completed");
        println!("4. Remove task");
        println!("5. Exit");
        println!("--------------------------------");

        print!("Choose an option: ");
        // force print without newline
        // flush is a method that flushes the buffer to the console
        // unwrap is a method that returns the value of the buffer
        io::stdout().flush().unwrap();

        // get user input
        let mut choice = String::new();
        // stdIn is a module that provides the stdin function
        // read_line is a method that reads a line from the console and stores it in a string
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim().parse() {
            Ok(1) => add_task(&mut tasks),
            Ok(2) => view_tasks(&tasks),
            Ok(3) => mark_task_as_completed(&mut tasks),
            Ok(4) => remove_task(&mut tasks),
            Ok(5) => break,
            // if the user enters an invalid option, print an error message
            // _ is a wildcard that matches any value
            _ => println!("Invalid option"),
        }
    }
}
