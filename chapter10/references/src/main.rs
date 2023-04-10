/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 * 
 * @brief This program demonstrates the way reference validation works in Rust.
 * 
 * @date 10 Apr 2023
 * @version 0.1
 */

fn main() {
    let s1: &str = "This is a very long string";
    let s2: &str = "This is a short string";

    // Display the longest string.

    println!("The longest string is \"{}\"", string_cmp(&s1, &s2));
} // end main()


// This function compares 2 strings and returns the reference
// to the longest one.
//
fn string_cmp<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // Check which string is longer.
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    } // end if
} // end string_cmp()
