use core::num;
/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program is a guessing game. It suggests a user to guess
 *        a number in the range from 1 to 100 in as many attempts, as
 *        user needs. On each step the program tells the player if
 *        the guess was too big or too small.
 *
 *        Finally, when the user guesses the number, the program
 *        informs them about it and shows the number of attempts used.
 *
 *
 * @version 0.1
 *
 */
use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    // Declare variables.

    let mut users_guess: String = String::new();

    // Generate a number to guess.

    let number_to_guess: u8 = rand::thread_rng().gen_range(1..=100);

    // This variables stores the number of attempts that the user
    // would have used to guess the number.

    let mut attempts: u32 = 0;

    // While the number is not guessed, continue the game.

    loop {
        // Clear the input string before reading the user's input in.

        users_guess.clear();

        // Ask a user to make a guess.

        println!("Make a guess!\n");

        // Read in user's input

        io::stdin()
            .read_line(&mut users_guess)
            .expect("Failed to read a line in");

        // Shadow user's input and parse it to an integer.
        // If the parsing is unsuccessful, then ignore the rest of the
        // code in the loop and start all over again.

        let users_guess: u8 = match users_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // end match

        // Count the number of attempts to guess the number.

        attempts += 1;

        // Check wheather the user's guess is too big or too small.
        // Inform the player about it.

        match users_guess.cmp(&number_to_guess) {
            // Check if the user's guess is too big.
            cmp::Ordering::Greater => println!("Your number is too big!\n"),

            // Check if the user's guess is too small.
            cmp::Ordering::Less => println!("Your number is too small!\n"),

            // Check if the user has guessed the number.
            cmp::Ordering::Equal => {
                // The user has guessed the right number.
                // Congratulate them and exit the program.

                println!("Congratulations!!!");
                println!("You guessed the number from the {attempts} attempt!:)");

                break;
            } // end cmp::Ordering::Equal
        } // end match
    } // end loop
} // end main()
