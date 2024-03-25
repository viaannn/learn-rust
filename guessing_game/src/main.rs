use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // defined new mutable varible with empty String value
    let mut guess = String::new();

    io::stdin()
        // read cli input, and assign to 'guess' mutable variable
        // this function is returning 'Result<usize, Error>' type
        .read_line(&mut guess)
        // when 'Result' type from '.read_line()' return an error, this 'expect()' will handle the error
        .expect("Failed to read line");

    // printing the guessed number using '{}' placeholders
    println!("You guessed: {guess}");
}