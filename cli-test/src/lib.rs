//! This library is a simple example of a library crate.
//! use cli-test::read_stdin;
//! 

use std::io::{BufRead, BufReader};

///This function reads a line from stdin and returns it as a a String
/// It will panic if it fails to read a line with a message "Failed to read line"
pub fn read_stdin() ->String{
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}

/// Adds two nummbers
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
