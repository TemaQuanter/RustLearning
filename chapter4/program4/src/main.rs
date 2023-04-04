/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program demonstrates the way slices work in Rust.
 *
 * @version 0.1
 */
use std::io;

fn main() {
    // Delcare variables.

    let mut s: String = String::new();

    let mut word_number: String = String::new();
    let word: &str;

    // Ask a user to enter a sentence.

    println!("Enter a sentence");

    // Read in user's input.

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read a string");

    // Ask a user what word they want to be found.

    println!("What word do you want to be printed out?");

    // Read in user's input.

    io::stdin()
        .read_line(&mut word_number)
        .expect("Failed to read in a string");

    // Try to convert a string to a number.

    let word_number: usize = match word_number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Failed to convert a string to a number");
        } // end Err
    }; // end match

    // Try to find the required word in the sentence.

    word = get_word(&s, word_number);

    // Display the searched for word.

    println!("{}", word);
} // end main()

// This function returns the slice to a particular word, that
// the user wants to get.
//
fn get_word(s: &str, index: usize) -> &str {
    // Declare variables.

    let mut word_counter: usize = 1;
    let mut word_start: usize = 0;

    let mut word_found: bool = false;

    let bytes: &[u8] = s.as_bytes();

    // Traverse the string and find the necessary word.

    for (i, &symbol) in bytes.iter().enumerate() {
        // Check if current symbol is the beginning of
        // the required word.

        if word_counter == index && !word_found {
            word_start = i;
            word_found = true;
        } // end if

        // Check if current symbol is a space.

        if symbol == b' ' {
            // Check if it is the end of the required word.

            if word_found {
                return &s[word_start..i];
            } // end if

            // Then it is the beginning of the next word.

            word_counter += 1;
        } // end if
    } // end for

    // At this stage the last word is the required word.

    return &s[word_start..];
} // end get_word()
