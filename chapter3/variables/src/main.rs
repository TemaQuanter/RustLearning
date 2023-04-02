fn main() {
    // Declare variables.

    let arr: [i32; 5] = [3, 4, 1, 1, 5];

    // Display each item of the array.

    for el in arr {
        println!("{}", el);
    } // end for

    // Display numbers from 1 to 4 inclusive.

    for el in 1..5 {
        println!("The value is {}", el);
    } // end for
} // end main()
