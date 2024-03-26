// Section 3.1: Variable and Mutability
// Learn about how variable and mutability works in rust

fn main() {
    // defined mutable variable with default assigned value is '5'
    let mut x = 5;

    // print out current 'x' value, which is '5'
    println!("The value of x is: {x}");

    // re-assign 'x' variable with new value '6'
    x = 6; 

    // print out updated  'x' value, which is '6'
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;
        
}
