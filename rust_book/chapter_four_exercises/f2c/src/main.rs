// Rust Book Chapter 3 very basic exercises
//
// Handle Fahrenheit to/from celsius
//
use std::io;


fn main() {
    let mut temp : f32 = 0.0;
    let mut input_line = String::new();
    io::stdin() 
        .read_line(&mut input_line) // actually read the line
        .expect("Failed to read line"); // which can fail, however

    let c: char = input_line
        .chars()
        .last()
        .unwrap();

    let temp_chars = &input_line[0..input_line.chars().count()-1];

    let t: f32 = temp_chars
        .trim() // ignore whitespace around input
        .parse() // convert to float
        .expect("Input not a float"); // which, again, can fail    io::stdin().read_line(&mut temp);
    println!("Input temperature is {t}");
}
