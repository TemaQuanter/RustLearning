use add_one::add_one;
use sub_one::sub_one;

fn main() {
    // Declare variables.

    let x: i32 = 11;

    // Perform mathematical operations on
    // a variable and display the result.

    println!("Current value is {}, the value after operation is {}", x, add_one(x));
    println!("Current value is {}, the value after operation is {}", x, sub_one(x));
} // end main()
