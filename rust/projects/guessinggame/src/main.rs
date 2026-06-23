use std::io;
use rand::Rng;

fn main() 
{
    println! ("Guess a number");

    println! ("Please input your guess");

    // Mutable variable declaration
    let mut guess = String::new();

    // Get user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println! ("You guess: {guess}");
}
