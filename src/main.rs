use std::io::stdin;

#[derive(Debug)]
struct Task {
    title: String,
}

fn add_task(Tasks: &mut Vec<Task>) {
    let mut title = String::new();

    println!("Enter task:");

    stdin()
        .read_line(&mut title)
        .expect("Failed to read task");

    let task = Task {
        title: title.trim().to_string(),
    };
    Tasks.push(task);

    println!("Task added!");
}

fn view_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }
    println!("\nTasks:");

    for (index, task) in tasks.iter().enumerate() {
        println!("{}: {}", index + 1, task.title);
    }
}

fn delete_task(tasks: &mut Vec<Task>) {
    view_tasks(tasks);

    if tasks.is_empty() {
        return;
    }
    let mut index = String::new();

    println!("Enter task number to delete:");

    stdin()
        .read_line(&mut index)
        .expect("Failed to read input");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number");
            return;
        }
    };

    if index == 0 || index > tasks.len() {
        println!("Task does not exist");
        return;
    }

    tasks.remove(index - 1);

    println!("Task deleted!");
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n====TODO APP ====");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Delete Tasks");
        println!("4. Exit");

        let mut choice = String::new();

        println!("Choose:");

        stdin()
            .read_line(&mut choice)
            .expect("Failed to read choice");

        match choice.trim() {
            "1" => add_task(&mut tasks),
            "2" => view_tasks(&tasks),
            "3" => delete_task(&mut tasks),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}
