/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This is a program for testing file ownership.
 *        In this case, the program works with a simple data type,
 *        so the ownership of the variable is not lost.
 *
 * @version 0.1
 */

fn main() {
    // Declare variables.

    let x: u32 = 5;

    // Inform a user if the variables is still not lost.

    println!("The value of x is {x}");

    // Call a function and pass the variable to it.

    display(x);

    // Check if the value is still not lost.

    println!("The value of x is {x}");
} // end main()

// This function displays an integer.
//
fn display(x: u32) {
    println!("Function call: {x}");
} // end display
