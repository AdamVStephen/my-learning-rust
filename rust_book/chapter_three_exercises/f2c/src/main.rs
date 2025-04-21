// Rust Book Chapter 3 very basic exercises
//
// Handle Fahrenheit to/from celsius
//
use std::io


fn main() {
    let mut temp : f32 = 0.0;
    let mut input_line = String::new();
    io::stdin() 
        .read_line(&mut input_line) // actually read the linei
        .expect("Failed to read line"); // which can fail, however

    let units: char = input_line
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_uppercase();

    let unitS: char = input_line
        .trim()
        .chars()
        .last()
        .unwrap();


    let mut conversion_coefficients_tup: (f32, f32) = (0.0, 0.0);
    let mut conversion_coefficients_arr: [f32; 2]= [0.0, 0.0];

    if (units == 'c') || (units == 'C') {
        conversion_coefficients_tup = (32.0, 9.0/5.0);
        conversion_coefficients_arr = [32.0, 9.0/5.0];
    } else if (units == 'f') {
        conversion_coefficients_tup = (32.0, 9.0/5.0);
        conversion_coefficients_arr = [32.0, 9.0/5.0];
    } else {
        conversion_coefficients_tup = (-1.0, -1.0);
        conversion_coefficients_arr = [-1.0, -1.0];
    }

    let temp_chars = &input_line[0..input_line.chars().count()-2];

    let t: f32 = temp_chars
        .trim() // ignore whitespace around input
        .parse() // convert to float
        .expect("Input not a float"); // which, again, can fail    io::stdin().read_line(&mut temp);

    println!("Input temperature is {t} in units {units}");

// (f32, f32) quite reasonably lacks a built-in pretty-printer : TODO create one.
//    println!("Conversion coefficients are {conversion_coefficients_tup}");
//
//    Notation for tuples is not tup[0] but rather tup.0
//
    println!("Conversion coefficients are {}\t{}",
        conversion_coefficients_tup.0,
        conversion_coefficients_tup.1);
    println!("Conversion coefficients are {}\t{}",
        conversion_coefficients_arr[0],
        conversion_coefficients_arr[1]);
}
