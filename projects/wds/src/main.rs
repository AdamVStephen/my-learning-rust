use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for at least one argument (the command)
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args...]", args[0]);
        std::process::exit(1);
    }

    // Extract the command and remaining arguments
    let command = &args[1];
    let command_args = &args[2..];

    // Create a vector to store the working directories
    let mut dirs : Vec<String> = Vec::new();
    
    //
    // Note : until used, it is an error to define a mutable variable
    // let mut my_dirs : Vec<String> = vec![];

    // Dispatch based on command
    match command.as_str() {
        "greet" => greet(command_args),
        "sum" => sum(command_args),
        "echo" => echo(command_args),
        "push" => push(&mut dirs, &command_args[0]),
        "pop" => pop(&mut dirs),
        _ => {
            eprintln!("Unknown command: {}", command);
            std::process::exit(1);
        }
    }
}

/// Push the current working directory on to the stack
fn push(vec: &mut Vec<String>, value: &String) {
    vec.push(value.to_string());
    list(vec);
}


/// Pop the current working directory off of the stack
//fn pop(vec: &mut Vec<String>) -> Option<String> {
fn pop(vec: &mut Vec<String>) {
    vec.pop();
    list(vec);
}

fn list(vec: &mut Vec<String>) {
    for dir in vec {
        println!("Directory\t{}", dir);
    }
}

/// Greet all names provided
fn greet(args: &[String]) {
    for name in args {
        println!("Hello, {}!", name);
    }
}

/// Sum all numbers provided
fn sum(args: &[String]) {
    let mut total = 0.0;
    for arg in args {
        match arg.parse::<f64>() {
            Ok(num) => total += num,
            Err(_) => eprintln!("Warning: '{}' is not a number, skipping.", arg),
        }
    }
    println!("Sum: {}", total);
}

/// Echo back the arguments
fn echo(args: &[String]) {
    println!("{}", args.join(" "));
}

