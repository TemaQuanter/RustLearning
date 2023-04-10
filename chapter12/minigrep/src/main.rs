/**
 * @file main.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This program is a mini-alternative to grep.
 *
 * @date 10 Apr 2023
 * @version 0.1
 *
 */
use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    // Declare variables.

    let args: Vec<String> = env::args().collect();

    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem with parsing arguments: {err}");
        process::exit(-1);
    });

    // Print out some information about what the program is going to do.

    println!("Searching for: \"{}\"", config.query);
    println!("In file: {}\n", config.file_name);

    // Display the content of the file.

    if let Err(e) = run(config) {
        // Error occured in the application.

        println!("Application error: {e}");

        process::exit(-1);
    } // end if let
} // end main()
