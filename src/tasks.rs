use std::io;

// #[derive(Debug)] lets us print the task with println!("{:?}", task)
#[derive(Debug)]

//struct is a keyword that defines a new data type
pub struct Task {
    //description: what the task is
    pub description: String,

    // completed: whether it's done or not (true/false)
    pub completed: bool,
    pub id: u32,
}

//------------------------------------------------------------//

//pub is a keyword that makes the function public
pub fn add_task(tasks: &mut Vec<Task>) {
    let mut description = String::new();
    println!("Enter the description of the task: ");

    //read_line is a method that reads a line from the console and stores it in a string
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read line");

    let task = Task {
        description: description.trim().to_string(),
        completed: false,
        id: tasks.len() as u32 + 1,
    };

    //push is a method that adds an element to the end of the vector
    tasks.push(task);
    println!("Task added successfully!");
}

//------------------------------------------------------------//

pub fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found");
        return;
    }

    //iter is a method that returns an iterator over the elements of the vector
    //enumerate is a method that returns a tuple of the index and the element
    for (i, task) in tasks.iter().enumerate() {
        let status = if task.completed {
            "Completed"
        } else {
            "Not Completed"
        };
        println!("{}: {} - {}", i + 1, task.description, status);
    }
}

//------------------------------------------------------------//

pub fn mark_task_as_completed(tasks: &mut Vec<Task>) {
    view_tasks(tasks);

    println!("Enter the id of the task you want to mark as completed: ");
    let mut id = String::new();

    //read_line is a method that reads a line from the console and stores it in a string
    io::stdin().read_line(&mut id).expect("Failed to read line");

    let id: u32 = id.trim().parse().expect("Please enter a valid number");

    //get_mut is a method that returns a mutable reference to the element at the given index
    // Some is a keyword that returns an optional value
    if let Some(task) = tasks.get_mut(id as usize - 1) {
        task.completed = true;
        println!("Task marked as completed!");
    } else {
        println!("Invalid task id");
    }
}

//------------------------------------------------------------//

pub fn remove_task(tasks: &mut Vec<Task>) {
    view_tasks(tasks);

    println!("Enter the id of the task you want to remove: ");
    let mut id = String::new();

    io::stdin().read_line(&mut id).expect("Failed to read line");

    // Parse the id and handle potential errors
    match id.trim().parse::<u32>() {
        // Ok is a keyword that returns an optional value
        //usize is an unsigned integer type that is the same size as the pointer size of the machine
        Ok(id) if id > 0 && (id as usize) <= tasks.len() => {
            tasks.remove((id - 1) as usize);
            println!("Task deleted!");
        }
        _ => {
            println!("Please enter a valid task id.");
        }
    }
}
