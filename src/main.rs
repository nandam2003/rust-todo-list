use std::{
    i32,
    io::{self, Write},
};

struct TodoItem {
    todo: String,
    completed: bool,
}

fn main() {
    let mut todos: Vec<TodoItem> = vec![];

    loop {
        println!("1. Add a todo");
        println!("2. Get all todos");
        println!("3. Mark a todo as done");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().expect("Error in flushing the msg out");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Sorry Unable the read the input.");

        match choice.trim().parse::<i32>() {
            Ok(num) => match num {
                1 => {
                    print!("Enter your Task: ");
                    io::stdout().flush().expect("Error flushing stdout");
                    let mut todo = String::new();
                    io::stdin().read_line(&mut todo).expect("Error had occured");
                    todos.push(TodoItem {
                        todo,
                        completed: false,
                    });
                    println!("Task Add Press '2' For getting all the Tasks.")
                }
                2 => {
                    if todos.is_empty() {
                        println!("List is Empty press '1' to add a To-Do.");
                        // io::stdout().flush().expect("Error flushing stdout");
                    } else {
                        for (i, t) in todos.iter().enumerate() {
                            println!("{}. Task: {} IsDone: {}", i + 1, t.todo, t.completed);
                        }
                    }
                }
                3 => {
                    print!("Enter Task no.: ");
                    io::stdout().flush().expect("Error flushing stdout");
                    let mut id = String::new();
                    io::stdin()
                        .read_line(&mut id)
                        .expect("Error in getting input.");
                    match id.trim().parse::<usize>() {
                        Ok(mut num) => {
                            num = num - 1;
                            if num < todos.len() {
                                todos[num].completed = !todos[num].completed;
                            } else {
                                println!("Out of range.")
                            }
                        }
                        Err(_) => println!("Invalid input"),
                    }
                }
                4 => break,
                _ => println!("Please enter a valid choice(1 - 4)"),
            },
            Err(e) => println!("Invalid input. Please enter a number.[Error: {e}]"),
        }
    }

    println!("Program Ended!! Thanks For you interaction.");
    println!();
}
