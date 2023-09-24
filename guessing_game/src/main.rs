use std::io;
use rand::Rng; // Rng trait defines methods that random number generators implement

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    /*
    rand::thread_rng function that gives us the random number generator
    gen_range we call the method on the random number generator
     */
    println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new(); // variable to store input

        // mut is mutable
        // remove mut for immutable

        io::stdin() // recieving user input std::io::stdin
            .read_line(&mut guess) // tells it what string to store user input in 
            // &mut makes reference to the guess mutable
            .expect("Failed to ead line");

        let guess: u32 = match guess.trim().parse() { // shadowing lets us reuse variables
            // u32 can only contain numerical data
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


/*
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2); // curly braces hld a calue in place
*/