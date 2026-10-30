
use std::env;

#[derive(Debug)]
struct Task {
    id: usize,
    title: String,
}

fn add_task(tasks: &mut Vec<Task>, title: String) -> Result<(), String> {
    if title.trim().is_empty() {
        return Err(String::from("Task title cannot be empty"));
    }

    let new_task = Task {
        id: tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1,
        title,
    };

    tasks.push(new_task);

    println!("TASKS {:?}", tasks);
    println!("Tasks added Successfully!");

    Ok(())
}

fn list_task(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No Task Added");
        return;
    }

    for task in tasks {
        println!("{}. {}", task.id, task.title);
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: usize) -> Result<(), String> {
    let index = tasks.iter().position(|task| task.id == id);

    match index {
        Some(index) => {
            tasks.remove(index);
            println!("Task Deleted Successfully");

            list_task(tasks);

            Ok(())
        }

        None => {
            Err(String::from("Task not found"))
        }
    }
}

fn run(arguments: &[String], tasks: &mut Vec<Task>) -> Result<(), String> {
    let command = arguments
        .get(1)
        .ok_or_else(|| String::from("Please provide a command"))?;

    match command.as_str() {
        "add" => {
            println!("The operation to be performed is Addition");

            let title = arguments
                .get(2)
                .cloned()
                .ok_or_else(|| String::from("Please provide a task title"))?;

            add_task(tasks, title)?;
        }

        "list" => {
            println!("The operation to be performed is List");

            list_task(tasks);
        }

        "delete" => {
            println!("The operation to be performed is Delete");

            let id: usize = arguments
                .get(2)
                .ok_or_else(|| String::from("Please provide a task ID"))?
                .parse()
                .map_err(|_| String::from("Invalid task ID"))?;

            delete_task(tasks, id)?;
        }

        _ => {
            return Err(String::from("Unknown command"));
        }
    }

    Ok(())
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    let arguments: Vec<String> = env::args().collect();

    if let Err(error) = run(&arguments, &mut tasks) {
        eprintln!("Error: {}", error);
    }
}
