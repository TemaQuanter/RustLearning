/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program opens a file with the name of a user
 *        and greets that user in a program.
 *
 * @date 8 Apr 2023
 * @version 0.1
 *
 */
use std::fs::File;
use std::io::{read_to_string, Read};

fn main() {
    // Declare variables.

    let mut user_name: String = String::new();
    let mut fl: File;

    // Try to open a file with the usersname.

    fl = match File::open("username.txt") {
        Ok(f) => f,
        Err(e) => panic!(
            "The opening of the file failed with the following error: {}",
            e
        ),
    }; // end match

    // Try to read from the file.

    fl.read_to_string(&mut user_name)
        .expect("Failed to read a line from the file");

    // Greet the user.

    println!("Hi, {}!", user_name);
} // end main()
