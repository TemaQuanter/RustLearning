/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This is a program that demonstrates the way
 *        references work in Rust.
 *
 * @version 0.1
 *
 */

fn main() {
    // Program 1

    // // Declare variables.

    // let mut s: String = String::from("Hello, world!");

    // // Create mutable references in different scopes.

    // {
    //     let a: &String = &mut s;
    //     let c: &String;

    //     println!("{}", a);
    // }

    // {
    //     let b: &String = &mut s;

    //     println!("{}", b);
    // }

    // end Program1

    // Program 2

    // Declare variables.

    let mut s: String = String::from("Hello, world!");

    let a: &String = &s;
    let b: &String = &s;
    let c: &String = &s;

    let m: &String;

    // The value cannot be assigned here, as it is one of the Rust reference
    // restrictions.
    //
    // m = &mut s;

    println!("{}\n{}\n{}", a, b, c);

    println!("{}\n{}\n{}\n", a, b, c);

    // After this point, neither of the immutable references above are used.
    // So, it is possible to assign a mutalbe references to a variable.

    m = &mut s;

    println!("{}", m);

    // end Program 2
} // end main()
