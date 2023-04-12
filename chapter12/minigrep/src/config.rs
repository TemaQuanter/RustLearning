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
    pub case_sensetive: bool
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

        // Declare varaibles.

        let case_sensetive: bool = std::env::var("CASE_SENSETIVE").is_ok();

        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
            case_sensetive
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

    for line in search(&content, &config.query, config.case_sensetive) {
        println!("{line}");
    } // end for

    Ok(())
} // end run

// This function performs the search of the query slice in
// the provided text.
//
fn search<'a>(content: &'a str, query: &str, case_sensetive: bool) -> Vec<&'a str> {
    // Declare variables.

    let mut result: Vec<&'a str> = Vec::new();
    let mut query: String = query.to_string();

    // Check if it is a non-case-sensetive search.

    if !case_sensetive {
        query = query.to_lowercase();
    } // end if

    let query: &str = query.as_str();
    
    // Iterate through each line of the text.

    for line in content.lines() {

        // Check if it is a non-case-sensetive search.
        // Check if current line contains the query stirng.

        if (!case_sensetive && line.to_lowercase().contains(query)) || (case_sensetive && line.contains(query)) {
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

        // Assert.

        assert_eq!(vec!["I love you", "Loved you!"], search(text, query, true));
    } // end searching_word()

    // This test makes sure that the case insensitive
    // search works.
    //
    #[test]
    fn insensitive_search() {
        let content: &str = "\
I love you
And
Have always
Loved you!";

        let query: &str = "love";

        // Assert.

        assert_eq!(vec!["I love you", "Loved you!"], search(content, query, false));
    } // end insensitive_search()
} // end mod tests
