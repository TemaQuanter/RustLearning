/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program demonstrates the usage of generics in Rust.
 *        This program reads in the numbers from a file, sorts them
 *        and displays the sorted array of numbers.
 */
use std::{fs::File, io::Read};

// Global constants.

// 30 is one of the most optimal values, when insertion sort works
// faster, rather than merge sort.

const MERGE_SORT_K: usize = 30;

fn main() {
    // Declare variables.

    let mut nums: Vec<i32> = Vec::new();
    let mut chars: Vec<char> = Vec::new();

    let file_name1: &str = "num_array.txt";
    let file_name2: &str = "char_array.txt";

    // Read in the data from files.
    // Parse the data from files to the arrays.

    read_in_array(file_name1, &mut nums);
    read_in_array(file_name2, &mut chars);

    // Sort the vectors.

    merge_sort(&mut nums);
    merge_sort(&mut chars);

    // Display the result.

    println!("Here are the sorted arrays");

    println!("{:?}", nums);
    println!("{:?}", chars);
} // end main()

// This functions reads in the data from a file to an array.
//
fn read_in_array<T: std::str::FromStr>(file_name: &str, arr: &mut Vec<T>)
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // Declare variables.

    let mut string_with_data: String = String::new();

    // Try to read from a file.

    match File::open(file_name) {
        Ok(mut fl) => {
            match fl.read_to_string(&mut string_with_data) {
                Ok(_) => (),
                Err(e) => panic!(
                    "Failed to read the data from a file with the following error: {}",
                    e
                ),
            } // end match
        } // end Ok
        Err(e) => panic!("Failed to open a file with the following error: {}", e),
    } // end match

    // Try to parse the data and store it in an array.

    for el in string_with_data.split_whitespace() {
        // Try to parse an element and add it to the vector.

        let el: T = el.parse().expect("Failed to parse an element");

        arr.push(el);
    } // end for
} // end read_in_array()

// This function merges 2 subvectors in a single vector.
//
fn merge<T: std::cmp::PartialOrd + std::clone::Clone + std::marker::Copy>(arr: &mut [T]) {
    // Declare variables.

    let upper_bound: usize = arr.len();
    let middle: usize = upper_bound / 2;

    let mut i: usize = 0;
    let mut j: usize = middle;

    let mut temp_arr: Vec<T> = Vec::new();

    // Merge 2 subvectors into 1.

    while i < middle || j < upper_bound {
        // Check if the first subvector is traversed to the end.
        // If so, then add the rest of the elements from the second
        // vector.

        if i == middle {
            // Add the rest of the elements from the second vector.

            while j < upper_bound {
                temp_arr.push(arr[j].clone());
                j += 1;
            } // end while

            // Merging is done, exit the loop.

            break;
        } // end if

        // Check if the second subvector is traversed to the end.
        // If so, then add the rest of the elements from the first
        // vector.

        if j == upper_bound {
            // Add the rest of the elements from the first vector.

            while i < middle {
                temp_arr.push(arr[i].clone());
                i += 1;
            } // end while

            // Merging is done, exit the loop.

            break;
        } // end if

        // Add the minimum of 2 values to the vector.

        if arr[i] < arr[j] {
            temp_arr.push(arr[i].clone());
            i += 1;
        } else {
            temp_arr.push(arr[j].clone());
            j += 1;
        } // end if
    } // end while

    // Copy the data from the temporary vector to the main one.

    for (ind, el) in temp_arr.iter().enumerate() {
        arr[ind] = *el;
    } // end for
} // end merge()

// This function sortes a vector of elements in a non-descending order.
// (merge sort)
//
fn merge_sort<T: std::cmp::PartialOrd + std::clone::Clone + std::marker::Copy>(arr: &mut [T]) {
    // Check if current case is the base case.

    if arr.len() <= MERGE_SORT_K {
        // Finish sorgin with insertion sort.

        insertion_sort(arr);

        return;
    } // end if

    // Declare variables.

    let arr_len: usize = arr.len();
    let middle: usize = arr_len / 2;

    // Sort each of the halves separately.

    merge_sort(&mut arr[0..middle]);
    merge_sort(&mut arr[middle..arr_len]);

    // Merge 2 sorted halves into 1 sorted array.

    merge(arr);
} // end merge_sort()

// This function sortes a vector of elemets in a non-descending order.
// insertions sort (is implemented in merger sort for enhanced efficiency).
//
fn insertion_sort<T: std::cmp::PartialOrd + std::clone::Clone + std::marker::Copy>(arr: &mut [T]) {
    // Declare variables.
    let mut t: T;

    // Traverse each of the elements in the provided array.

    for i in 1..arr.len() {
        // Place current element to its place in the ordered vector.

        for j in (1..(i + 1)).rev() {
            // Check if it is necessary to swap the elements.

            if arr[j] < arr[j - 1] {
                // Swap the elements.

                t = arr[j];
                arr[j] = arr[j - 1];
                arr[j - 1] = t;
            } else {
                // There is no swap needed.
                // The element is already on its place.

                break;
            } // end if
        } // end for
    } // end for
} // end merge_sort()
