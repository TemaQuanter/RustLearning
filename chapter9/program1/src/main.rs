use std::fs::File;

fn main() {
    // Declare variables.

    let file_name: &str = "test.txt";

    // Try to open a file.

    let fl: File = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => {
            println!("Although the file you are searching for does not exist,");
            println!("the program will try to create it.");

            match File::create(file_name) {
                Ok(f1) => {
                    println!("The program managed to create the file");
                    f1
                }
                Err(_) => panic!("It is neither possible to open file, nor create it"),
            } // end match
        } // end match Err
    }; // end fl
} // end main()
