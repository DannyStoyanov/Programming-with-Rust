use std::fs;
use std::fs::File;
use std::fmt;

//use std::io::Read;
use std::io;
use std::io::prelude::*;
use std::result;
use std::num;

/*enum Result<T, E> {
    Ok(T),
    Err(E),
}*/

fn direct_quotes() {
    // reading from file:
    let mut file = match File::open("quotes.txt") {
        Ok(f) => f,
        Err(e) => panic!("ERROR! Couldn't read file! {}", e),
    };
    /*
    // safe the content:
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    */

    // without unwrap:
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents); // .unwrap(), Result<usize, io::Error>

    // print the content:
    println!("{}", contents)
}

fn quotes() -> Result<String, io::Error> {
    let mut quotes = (File::open("quotes.txt"))?;

    let mut contents = String::new();

    (quotes.read_to_string(&mut contents))?;

    Ok(contents)
}

struct CustomError {
    message: String,
}

impl From<io::Error> for CustomError {
    fn from(e: io::Error) -> Self {
        CustomError { message: format!("IO Error: {}", e) }
    }
}

impl From<num::ParseIntError> for CustomError {
    fn from(e: num::ParseIntError) -> Self {
        CustomError { message: format!("ParseError: {}", e)}
    }
}

fn read_numbers_from_file() -> Result<i32, CustomError> {
    let mut numbers = File::open("numbers.txt").
    or_else(|_error| File::open("passable.txt")).
    or_else(|_error| File::open("okay_i_guess.txt"))?;

    let mut contents = String::new();
    (numbers.read_to_string(&mut contents))?;

    let n = match contents.lines().next() {
        Some(first_value) => (first_value.parse::<i32>())?,
        None => 0,
    };

    Ok(n)
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// clean version:
fn clean_read_numbers_from_file() -> Result<String, CustomError> {
    let mut numbers = String::new();
    numbers.push_str(&fs::read_to_string("numbers.txt")?);
    Ok(numbers)
}

fn main() {
    match clean_read_numbers_from_file() {
        Ok(contents) => println!("{}", contents),
        Err(e) => panic!("{}", e),
    }

    println!("-----------------------------------");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_from_main() {
        assert_eq!(2 + 2, 4);
    }
}
