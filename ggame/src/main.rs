use std::io;



fn main() {
    println!("Initial exercise : guessing game");
    println!("What range ?");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read a line");

    println!("You guessed {}", guess);
}
