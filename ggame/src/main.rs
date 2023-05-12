use std::io;
use std::cmp::Ordering;



fn main() {
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
