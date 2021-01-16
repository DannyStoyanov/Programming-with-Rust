use std::io::Error;
use std::fmt::Debug;
use std::fmt;
/*
pub fn skip_next(input: &str, target: char) -> Option<&str> {
    let len = input.len();
    for (i, c) in input.chars().enumerate() {
        if c == target {
            let res = &input[(i+1)..len];
            return Some(res);
        } 
    }
    None
} */
pub fn skip_next(input: &str, target: char) -> Option<&str> { // works with UTF-8 chars
    let mut char_indices = input.char_indices();
    let mut temp = char_indices.next();
    let len = input.len();
    while temp != None {
        if temp.unwrap().1 == target {
            let res = &input[temp.unwrap().0 + temp.unwrap().1.len_utf8() .. len];
            return Some(res);
        }
        temp = char_indices.next();
    } 
    None
}
/*
pub fn take_until(input: &str, target: char) -> (&str, &str) {
    let empty = "";
    let len = input.len();
    for (i, c) in input.chars().enumerate() {
        if c == target {
            let first = &input[0..i];
            let second = &input[i..len];
            return (first, second);
        }
    }
    (input, empty)
}*/
pub fn take_until(input: &str, target: char) -> (&str, &str) { // works with UTF-8 chars
    let empty = "";
    let mut char_indices = input.char_indices();
    let mut temp = char_indices.next();
    while temp != None {
        if temp.unwrap().1 == target {
            let (first, second) = input.split_at(temp.unwrap().0);
            return (first, second);
        }
        temp = char_indices.next();
    } 
    (input, empty)
}
pub fn take_and_skip(input: &str, target: char) -> Option<(&str, &str)> {
    let res = take_until(input, target);
    match res.1 {
        "" => None,
        _ =>     Some((res.0, skip_next(res.1, target).unwrap()))
 
    }
}

#[derive(Debug)]
pub enum CsvError {
    IO(std::io::Error),
    ParseError(String),
    InvalidHeader(String),
    InvalidRow(String),
    InvalidColumn(String),
}

impl fmt::Display  for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CsvError::IO(E) => write!(f, "-->ERROR-MESSAGE|IO: {}", E),
            CsvError::ParseError(text) => write!(f, "-->ERROR-MESSAGE|PARSE-ERROR: {}", text),
            CsvError::InvalidHeader(text) => write!(f, "-->ERROR-MESSAGE|INVALID-HEADER: {}", text),
            CsvError::InvalidRow(text) => write!(f, "-->ERROR-MESSAGE|INVALID-ROW: {}", text),
            CsvError::InvalidColumn(text) => write!(f, "-->ERROR-MESSAGE|INVALID-COLUMN: {}",text),
        }
    }
}

use std::collections::HashMap;

type Row = HashMap<String, String>;

use std::io::BufRead;

pub struct Csv<R: BufRead> {
    pub columns: Vec<String>,
    reader: R,
    selection: Option<Box<dyn Fn(&Row) -> Result<bool, CsvError>>>,
}

fn remove_quotes(data: &str) -> &str {
    let res = data.trim();
    &res[1.. res.len()-1]
}

fn contains_dup(buffer: Vec<String>) -> bool {
    let mut res: Vec<&str> = vec![];
    for el in &buffer {
        if res.contains(&el.trim()) == true {
            return true;
        }
        res.push(&el.trim());
    }
    return false;
}

fn convert_header(header: &str) -> Option<Vec<String>> {
    let mut words = vec![];
    let mut pair: Option<(&str, &str)> = take_and_skip(header, ',');
    let mut last = "";
    while pair != None { 
        words.push(pair.unwrap().0.trim().to_string());
        if take_and_skip(pair.unwrap().1, ',') == None {
            last = pair.unwrap().1;
        }
        pair = take_and_skip(pair.unwrap().1, ',');
    }
    if words.len() == 0 {
        words.push(header.trim().to_string());
    }
    else {
        words.push(last.trim().to_string());
    }
    match words.len() {
        0 => return None,
        _ => return Some(words),
    }
}

fn in_quotes(argument: &str) -> bool {
    let chars: Vec<char> = argument.trim().chars().collect();
    if chars.len() < 2 {
        return false
    }
    chars[0] == '\"' && chars[chars.len()-1] == '\"'
} 

use std::io::Write;
use std::io::{self, BufReader};
use std::io::prelude::*;

impl<R: BufRead> Csv<R> {
    pub fn new(mut reader: R) -> Result<Self, CsvError> {
        //let mut source = BufReader::new(reader);
        let mut buf = String::new();
        match reader.read_line(&mut buf)  {
            Ok(0) => return Err(CsvError::InvalidHeader(String::from("Empty header!"))),
            Err(E)  => return Err(CsvError::IO(E)),
            _ => {
                let first_line = take_until(&buf, '\n');
                let header =  convert_header(&first_line.0).unwrap();
                match contains_dup(header.clone()) {
                    true => return Err(CsvError::InvalidHeader(String::from("Contains duplicated values!"))),
                    _    => return Ok(Csv {columns: header, reader: reader, selection: None}),
                }
            }
        }
    }

    pub fn parse_line(&mut self, line: &str) -> Result<Row, CsvError> {
        let len = self.columns.len();
        let mut temp = line.trim();
        let mut row = Row::new(); 
        for i in 0..len {
            let pair = take_and_skip(temp, ',');
            match pair {
                None => {
                    if i != len-1 {
                        return Err(CsvError::InvalidRow(String::from("-->ERROR-MESSAGE|INVALID-ROW: Keys more than arguments!")));
                    }
                    else {
                        if in_quotes(&temp) == true {
                            row.insert(String::from(self.columns[i].clone()), String::from(remove_quotes(&temp)));
                            return Ok(row);
                        }
                        else {
                            return Err(CsvError::InvalidRow(String::from("-->ERROR-MESSAGE|INVALID-ROW: Argument not starting or ending with '\"' !")));
                        }
                    }
                },
                Some((current, next)) => {
                    if in_quotes(current) == true {
                        if i != len-1 {
                            row.insert(String::from(self.columns[i].clone()), String::from(remove_quotes(&current)));
                            temp = next;
                        }
                        else {
                            return Err(CsvError::InvalidRow(String::from("-->ERROR-MESSAGE|INVALID-ROW: Arguments more than keys!")));
                        }
                    }
                    else {
                        return Err(CsvError::ParseError(String::from("Invalid row! Argument not starting or ending with '\"' !")));
                    }
                },
            }
        }
        Ok(row)
    }
    pub fn apply_selection<F>(&mut self, callback: F)
    where F: Fn(&Row) -> Result<bool, CsvError> + 'static
    {
        self.selection = Some(Box::new(callback));
    }
    pub fn write_to<W: Write>(mut self, mut writer: W) -> Result<(), CsvError> {
        let len = self.columns.len();
        // print header:
        for i in 0 .. len {
            if i != len - 1 { 
                    match write!(&mut writer, "{}, ", self.columns[i]) {
                    Ok(_) => (),
                    Err(E) => return Err(CsvError::IO(E)),
                };
            }
            else { 
                match write!(&mut writer, "{}\n", self.columns[i]) {
                    Ok(_) => (),
                    Err(E) => return Err(CsvError::IO(E)),
                };
            }
        }

        // print rows 
       
        let mut buf = String::new();
        let mut line = self.reader.read_line(&mut buf);
        loop {
            match line {
               Ok(0) => return Ok(()),
               Err(E)  => return Err(CsvError::IO(E)),
               _ => {
                   let row = self.parse_line(&buf);
                   match row {
                       Err(E) => return Err(CsvError::ParseError(String::from(""))),
                       Ok(_) => {
                        let mut i = 0;
                        for val in self.columns.clone() {
                            i = i + 1;
                            if i != self.columns.len() {
                                match write!(&mut writer, "\"{}\", ", self.parse_line(&buf).unwrap()[&val].as_str()) {
                                    Ok(_) => (),
                                    Err(E) => return Err(CsvError::IO(E)),
                                };
                            }
                            else {
                                match write!(&mut writer, "\"{}\"\n", self.parse_line(&buf).unwrap()[&val].as_str()) {
                                    Ok(_) => (),
                                    Err(E) => return Err(CsvError::IO(E)),
                                };
                            }
                        }
                        buf.clear();
                        line = self.reader.read_line(&mut buf);
                       }
                   };
               }
            }
        }
        Ok(())
    }
}

impl<R: BufRead> Iterator for Csv<R> {
    type Item = Result<Row, CsvError>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.reader.read_line(&mut buf)  {
            Err(E) => return Some(Err(CsvError::IO(E))),
            Ok(0) => return None,
            _ => {
                let row = self.parse_line(&buf);
                match row {
                    Err(E) => return Some(Err(E)),
                    Ok(_) => {
                        match self.selection {
                            None => return Some(Ok(self.parse_line(&buf).unwrap())),
                            _ => {
                                match self.selection.as_ref().unwrap()(&row.unwrap()) {
                                    Err(E) => return Some(Err(E)),
                                    Ok(false) => return self.next(),
                                    Ok(true) => return Some(Ok(self.parse_line(&buf).unwrap())),
                                 }
                            }
                        }
                    },
                }
            },
        }
    }
}


// --------------------------------------------------------------------------------------------------------------------
fn main() {
   // Подготвяме данните:
    let reader = BufReader::new(r#"
    name, age, birth date
    "Douglas Adams", "42", "1952-03-11"
    "Gen Z. Person", "20", "2000-01-01"
    "Ada Lovelace", "36", "1815-12-10"
    "#.trim().as_bytes());

    // Конструираме си CSV-то:
    let mut csv = Csv::new(reader).unwrap();

    // Инсталираме условието -- само редове с възраст над 30 ще останат:
    /*csv.apply_selection(|row| {
    let age = row.get("age").ok_or_else(|| CsvError::InvalidColumn(String::from("age")))?;
    let age = age.parse::<u32>().map_err(|_| CsvError::ParseError(String::from(age)));

    Ok(age.unwrap() > 30)
    });*/
    // Итерираме през резултата:
    while let Some(row) = csv.next() {
    println!("{:?}", row.unwrap().get("name"));
    // => Some("Douglas Adams")
    // => Some("Ada Lovelace")
    }
}
