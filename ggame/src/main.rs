use std::io;
use std::cmp::Ordering;

// Variables and Types
//
const PI_INT :i32 = 314;

fn show_pi() {
    println!("Pi is approximately {} to an engineer", PI_INT/100);
    let pi :f32 = PI_INT as f32;
    let pi :f32 = (PI_INT as f32)/100.0;
    println!("Pi is approximately {} to an accountant", PI_INT as f32/100.00);

    // Build up some knowledge of function signatures
    //

    // Return the integer and fractional parts separately
    let pi : (i32, f32) = calc_pi_i_f("hello");
    //let pi_frac : &str = &String::from(pi.1)[2..];
    let pi_frac : &str = &format!("{}",pi.1)[2..];
    println!("Pi is approximately {},{} computed by calc_pi", pi.0, pi_frac); 

    println!("Select a method by which to calculate pi");
    let mut method = String::new();
    io::stdin().read_line(&mut method)
        .expect("Failed to read a method");
    let method = method.trim();
    let pi : (f32, &str) = calc_pi(&method);

    // Todo : anonymous named tuple type to get a hash style going

    println!("Pi computed by method |{}| = {}", pi.1, pi.0);



}

//let pi_methods : (str, str) = ("hard_coded", "rational_approximation");

fn calc_pi(method: &str) -> (f32, &str) {
    match method {
        "hard" => {return (3.14159265, "hard_coded");},
        "rat" => {return ((22.0/7.0) as f32, "rational_approximation");},
        &_ => {return (0.000, "default_error");},
    };
    //(0.000, "default_error")
}

fn calc_pi_i_f(method: &str) -> (i32, f32) {
    (3, 0.14159)
}

// Debugging cghecks

fn demo_debug() {

    println!("demo_debug");
//    println!(">>> demo_debug");
 //   println!(">>> demo_debug\n\n");
    // 
    //
}


// Stack Container Basics

fn demo_tups() {

    println!(">>> demo_tups\n\n");
    let immut_fruit_tup : (&str, &str, &str) = ("apple", "banana", "cherry");

    let immut_fruit_arr = ["apple", "banana", "cherry"];

    let immut_fruit_container = immut_fruit_arr;

    for fruit in immut_fruit_container.iter() {
        println!("And another fruit : {}", fruit);
    }

    println!("We can print all the fruit very efficiently with spaces between using {{:?}}");
    println!("{:?}", immut_fruit_container);

    println!("<<< demo_tups");
}

fn demo_guessing_game(){
println!("Initial exercise : guessing game");
    println!("What range ?");

    let pass_number = 314;


    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read a line");

        // Cast the shadow guess to integer type for comparison
        //
        // .expect seems unable to take a formatting construct
        // .expect("expr {}", foo); is rejected
        /*
           let guess :i32 = guess.trim().parse()
           .expect("Looking for an integer");
           */
        let guess :i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>{println!("Not a number"); continue},
        };
        println!("You guessed {}", guess);
        //
        // 
        // Unexpectedly - the use of guess.cmp has changed ownerhship


        match guess.cmp(&pass_number) {
            Ordering::Less => println!("Too big"),
            Ordering::Greater => println!("Too small"),
            Ordering::Equal => {println!("Just right"); break;}
        }

        if pass_number != guess {
            println!("Not the secret, guess again.");
        } else {
            println!("Congratulations {} ", pass_number);
            return;
        }

    }
}

// Demo of some structs

struct Dog {
    breed:String,
    name:String,
}

fn demo_dogs() -> Dog {
    let breed = String::from("Spaniel");
    let name = String::from("Trixie");
    let trixie = Dog { breed, name};
    trixie
}

fn main() {
demo_debug();
 //   show_pi();
   demo_tups(); 
//    demo_guessing_game();
    let doggie = demo_dogs();
    println!("The doggie {} is a {}", doggie.name, doggie.breed);
}
