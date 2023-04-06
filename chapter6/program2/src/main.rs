/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program demonstrates the way Options work in Rust.
 *
 * @date 6 Apr 2023
 * @version 0.1
 */

fn main() {
    // Declare variables.

    let a: i32 = -13;
    let b: Option<i32> = Some(32);

    let result_sum: i32;
    let result_dif: i32;

    // Try to sum up 2 numbers.

    result_sum = match b {
        Some(num) => a + num,
        None => 0,
    }; // end match

    // Try to substract 2 numbers.

    result_dif = if let Some(num) = b { a - num } else { 0 };

    // Display the result.

    println!("a + b = {}", result_sum);
    println!("a - b = {}", result_dif);
} // end main()
