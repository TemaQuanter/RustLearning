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

    // Display the content found in
    // a formatted way.

    for line in search(&content, &config.query) {
        println!("{line}");
    } // end for

    Ok(())
} // end run

// This function performs the search of the query slice in
// the provided text.
//
fn search<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    // Declare variables.

    let mut result: Vec<&'a str> = Vec::new();
    
    // Iterate through each line of the text.

    for line in content.lines() {
        // Check if current line contains the query stirng.

        if line.contains(query) {
            // Current line contains the query string.

            result.push(line);
        } // end for
    } // end for

    result
} // end search()


#[cfg(test)]
mod tests {
    use super::*;

    // This test makes sure that the program finds the
    // searched for slice in the text.
    //
    #[test]
    fn searching_word() {
        // Declare variables.

        let text: &str = "\
I love you
And
Have always
Loved you!";

        let query: &str = "ove";

        assert_eq!(vec!["I love you", "Loved you!"], search(text, query));
    } // end searching_word()
} // end mod tests
