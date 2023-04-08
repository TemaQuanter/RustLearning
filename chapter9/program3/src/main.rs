/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program opens a file, reads a line of
 *        numbers, then it calculates the sum of the numbers
 *        and displays it to the standart output.
 *
 * @date 8 Arp 2023
 * @version 0.1
 *
 */
use std::fs::File;
use std::{io, io::Read};

fn main() {
    // Declare variables.
    let s: String;
    let file_name: &str = "array.txt";
    let mut vc: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    // Try to read in the data from a file.

    s = match read_from_file(file_name) {
        Ok(s) => s,
        Err(e) => panic!("Failed to read from file. The error message is: {}", e),
    }; // end match

    // Split the read string by spaces, convert elements into integers.
    // Add all the elements to the vector.

    for el in s.split_whitespace() {
        // Parse an element into integer.

        let el: i32 = el.parse().expect("Failed to parse an element into integer");

        // Ad an element into a vector.

        vc.push(el);
    } // end for

    // Calculate the sum of the elements.

    for el in &vc {
        sum += el;
    } // end for

    // Display the result.

    println!("The sum of the elements is {}", sum);
} // end main()

// This function returns the result of reading
// some data from a file.
//
fn read_from_file(file_name: &str) -> Result<String, io::Error> {
    // Declare variables.
    let mut s: String = String::new();

    // Try to open a file and read data.

    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
} // end read_from_file()
