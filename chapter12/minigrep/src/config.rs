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
    pub case_sensetive: bool,
} // end struct Config

impl Config {
    // This function makes an attempt to build a config for
    // the program.
    //
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skip the name of the program.

        args.next();

        // Declare varaibles.

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("The search for word is not specified"),
        }; // end query

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("The file name is not specified"),
        }; // end file_name

        let case_sensetive: bool = std::env::var("CASE_SENSETIVE").is_ok();

        Ok(Config {
            query,
            file_name,
            case_sensetive,
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

    // Check if it is a non-case-sensetive search.

    let query: String = if case_sensetive {
        query.to_string()
    } else {
        query.to_lowercase()
    }; // end query

    // Filter all the elements and get the vector of searched for values.

    // Check if it is a non-case-sensetive search.

    if case_sensetive {
        content
            .lines()
            .into_iter()
            .filter(|line| line.contains(&query))
            .collect()
    } else {
        content
            .lines()
            .into_iter()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
    } // end if
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

        assert_eq!(
            vec!["I love you", "Loved you!"],
            search(content, query, false)
        );
    } // end insensitive_search()
} // end mod tests
