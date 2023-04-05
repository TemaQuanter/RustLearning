/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program is a simple demonstration of using structs
 *        in Rust.
 *
 * @date 4 Apr 2023
 * @version 0.1
 */

// Declare structure templates.

// This structure represents a user.
//
struct User {
    active: bool,
    username: String,
    first_name: String,
    last_name: String,
    age: u8,
} // end struct template User

fn main() {
    // Declare variables

    let user1 = User {
        active: true,
        username: String::from("Max123"),
        first_name: String::from("Max"),
        last_name: String::from("Murphy"),
        age: 23,
    }; // end User

    let user2: User = User {
        active: false,
        username: String::from("Kiss1"),
        first_name: String::from("Kate"),
        last_name: String::from("Mask"),
        age: 17,
    }; // end User

    // Display the users.

    display_user(&user1);
    display_user(&user2);
} // end main()

// This program displays a user in a formatted way.
//
fn display_user(user: &User) {
    // Display some information about a user
    // in a formatted way.

    println!();
    println!("Active: {}", user.active);
    println!("Username: {}", user.username);
    println!("First name: {}", user.first_name);
    println!("Last name: {}", user.last_name);
    println!("Age: {}", user.age);
    println!();
} // end display_user
