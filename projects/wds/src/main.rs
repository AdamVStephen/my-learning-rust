use std::env;

use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use serde::{Deserialize, Serialize};

/// Serializes a Vec<String> to a file in JSON format.
fn save_vec_to_file(vec: &Vec<String>, path: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, vec)?;
    Ok(())
}

/// Deserializes a Vec<String> from a JSON file.
fn load_vec_from_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let vec = serde_json::from_reader(reader)?;
    Ok(vec)
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

fn main() -> std::io::Result<()>{
    // Define the path to the wds dirs
    let wds_path = "~/.config/wds.json";

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
    //let mut dirs : Vec<String> = Vec::new();

    // Note : until used, it is an error to define a mutable variable
    // let mut my_dirs : Vec<String> = vec![];

    // When creating the vector from serdes pair, no need to
    // pre-create an object to be populated.

    let mut dirs = load_vec_from_file(wds_path)?;

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

    let _ = save_vec_to_file(&dirs, wds_path);

    Ok(())
}


