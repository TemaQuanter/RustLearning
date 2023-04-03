/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program demonstrates the way ownership works in Rust.
 *
 * @version 0.1
 */

fn main() {
    // Declare variables.

    let s1: String = String::from("Hello, world!");

    // This would move the pointer to the value to the string "s2".
    //
    // let s2: String = s1;
    
    let s2: String = s1.clone();

    // This would not work.
    //
    // take_ownership(s1);

    take_ownership(s1.clone());

    println!("{} {}", s1, s2);
} // end main()


// This function takes the ownership of a string.
//
fn take_ownership(s: String) {

    println!("The ownership of the string \"{}\" is taken", s);

} // end take_ownership()
