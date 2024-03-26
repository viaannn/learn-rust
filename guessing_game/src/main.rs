// Section 2: Programming a Guessing Name
// Create program to guessing a random number

// import standard library i/o
use std::io;

// import 'Rng' random generator function from rand library
use rand::Rng;

// import standart library to compare and import Ordering enum to make a condition
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // generate random numbers, between 1 to 100
    // 'thread_rng()' fn is for gives us particular random generator that we will use
    // 'gen_range()' fn i for takes a range expression to generate random numbers based on the parameters
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // loop to keep the program is running
    loop {
        println!("Please input your guess.");

        // defined new mutable varible with empty String value
        let mut guess = String::new();

        io::stdin()
            // read cli input, and assign to 'guess' mutable variable
            // this function is returning 'Result<usize, Error>' type
            .read_line(&mut guess)
            // when 'Result' type from '.read_line()' return an error, this 'expect()' will handle the error
            .expect("Failed to read line");

        // create new variable with existing 'guess' value that has been convert to int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // printing the guessed number using '{}' placeholders
        println!("You guessed: {guess}");
        println!("Result: ");

        // compare 'guess' input value and 'secret_number' if it's less, greater or equal
        match guess.cmp(&secret_number){
            Ordering::Less => print!("Too small! \n\n"),
            Ordering::Greater => print!("Too big! \n\n"),
            Ordering::Equal => {
                // if win the game, it's gonna break the loop to exit the program
                print!("You win! \n\n");
                break;
            }
        }  
    }
}