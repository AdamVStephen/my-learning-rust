// Fibonacci - get the nth number
//
use std::io;
fn fib(n: i32) -> i32 {
    if n==1 {
        1}
    else if n==2 {
        1}
    else {
        fib(n-1) + fib(n-2)
    }
}

fn main() {
    //    let mut which : 132 = 1;
    let mut input_line = String::new();
    io::stdin() 
        .read_line(&mut input_line) // actually read the line
        .expect("Failed to read line"); // which can fail, however

    let which: i32 = input_line
        .trim() // ignore whitespace around input
        .parse() // convert to float
        .expect("Input not an integer"); // which, again, can fail    io::stdin().read_line(&mut temp);

//    for i in 1..10 {
//        println!("Fib number {} is {}", i, fib(i));
//    }

    println!("The {}th Fibonacci number is {}", which, fib(which));
}
