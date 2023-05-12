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
    let pi_frac : &str = String::from(pi.1)[2..];
    println!("Pi is approximately {},{} computed by calc_pi", pi.0, pi_frac); 
}

//let pi_methods : (str, str) = ("hard_coded", "rational_approximation");

fn calc_pi(method: &str) -> (f32, &str) {
    (3.14159265, "hard_coded")
}

fn calc_pi_i_f(method: &str) -> (i32, f32) {
    (3, 0.14159)
}

fn main() {
    show_pi();
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

    if (pass_number != guess) {
        println!("Not the secret, guess again.");
    } else {
        println!("Congratulations {} ", pass_number);
        return;
    }

    }
}
