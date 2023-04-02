use rand::Rng;
use std::io;

/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program is a guessing game. The logic of the program is the
 *        following.
 *
 *        1. At the very beginning the program thinks of a number
 *           that the player will have to guess in the process of the game.
 *
 *        2. The player is given 3 attempts to guess the number between
 *           1 and 10 inclusive.
 *
 *
 *        3. Player tries to guess the number and after each attempt the user
 *           is shown the verdict of their guess.
 *
 * @version 0.1
 *
 */

fn main() {
    // Declare variables.

    let number_to_guess: u8 = rand::thread_rng().gen_range(1..=10);
    let attempts: u8 = 3;

    let mut users_guess: String = String::new();

    // Greet the user and explain the rules of the game.

    println!(
        "Welcome to the guessing game! You have {} attempts to\n",
        attempts
    );
    println!("make a guess. The program has already come up with the number\n");
    println!("you need to guess\n\n");

    println!("The number is from 1 to 10 inclusive.\n");

    // Give a user several attemps to guess the number.

    for i in 0..attempts {
        // Ask a user to enter the number.

        println!("Please, make a guess!");

        // Clear the variables that stores user's input, so that
        // the next input would not be appended to the current one.

        users_guess.clear();

        // Read in user's input.

        io::stdin()
            .read_line(&mut users_guess)
            .expect("Failed to read a line");

        // Shadow the user's guess with an integer.

        let users_guess: u8 = users_guess
            .trim()
            .parse()
            .expect("The number was not converted successfully");

        
        // Format the output.

        println!();

        // Check if the user has guessed the number.

        if users_guess == number_to_guess {
            // The user has guessed the number correctly.
            // Congratulate them and exit the program.

            println!("Contratulations!\n\n");
            println!("You guessed the number!!!\n");

            break;
        } else if i < attempts - 1 {
            // The user did not guess the number correctly.
            // Moreover, they still have some attempts left.

            println!("You guess was not right:(\n");
        } else {
            // The user did not guess the number correctly.
            // Moreover, they are run out of attempts.

            println!("Sorry, but not this time:(\n");
            println!("By the way, the right number was {}\n", number_to_guess);
        } // end if
    } // end for
} // end main()
