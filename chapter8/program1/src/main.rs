/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 * 
 * @brief This program demonstrates the usage of some very popular
 *        collections in Rust.
 * 
 *        1. Strings
 *        2. HashMaps
 *        3. Vectors
 * 
 * @date 8 Apr 2023
 * @version 0.1
 * 
 */

use std::collections::HashMap;

fn main() {
    // Declare variables.

    let mut v: Vec<i32> = vec![3, -2, 3, 5];
    let mut v_sum: i32 = 0;

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");

    let s3: String = format!("{}{}", s1, s2);

    let mut hm: HashMap<String, Vec<u8>> = HashMap::new();

    // Manipulate the values of the vector.

    v[1] = 3;
    v.push(7);

    // Traverse the vector and calculate the sum of its variables.

    for el in &v {
        v_sum += el;
    } // end for

    // Manipulate the hash-map.

    hm.insert(String::from("Alex"), vec![2, 3, 5, 3, 5]);
    hm.insert(String::from("Mary"), vec![5, 5, 4]);

    // Display the results of operations above.

    println!("Sum: {}", v_sum);
    println!("\"{}\" + \"{}\" = \"{}\"", s1, s2, s3);

    // Display the marks of students.

    for (key, value) in &hm {
        // Declare variables.

        let marks: usize = value.len();

        // Introduce a student.

        println!("{} got the following {} marks:", key, value.len());

        // Display the marks of a particular student.

        for (ind, mark) in value.iter().enumerate() {
            // Check if it is the last mark to display.
            // Format the output accordingly.

            if ind == marks - 1 {
                // This is the last element.

                println!("{}", mark);
            } else {
                // This is not the last element.

                print!("{}, ", mark);
            } // end if
        } // end for
    } // end for
} // end main()
