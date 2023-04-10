/**
 * @file config.rs
 * @author Artem Mikheev (github: TemaQuanter)
 *
 * @brief This is the main library that provides
 *        some functionality to the program.
 *
 * @date 10 Apr 2023
 * @version 0.1
 */
use std::error::Error;
use std::fs;

// This structure stores some config information for the program.
//
pub struct Config {
    pub query: String,
    pub file_name: String,
} // end struct Config

impl Config {
    // This function makes an attempt to build a config for
    // the program.
    //
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if there is a valid number of arguments passed
        // to the program.
        if args.len() != 3 {
            // The number of elements is invalid.

            return Err("The number of arguments should be 2");
        } // end if

        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
        })
    } // end build()
} // end impl Config

// This function performs the main logic of the program.
//
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Declare variables.

    let content: String;

    // Try to open a file and read the data from it.

    content = fs::read_to_string(&config.file_name)?;

    // Display the content of the file in
    // a formatted way.

    println!("Content of \"{}\"\n\n{}", config.file_name, content);

    Ok(())
} // end run
