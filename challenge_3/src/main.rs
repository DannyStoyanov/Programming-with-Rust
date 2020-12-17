use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bigint {
    pub digits: Vec<u8>,
}

impl FromStr for Bigint {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());

        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                digits.push(digit as u8);
            } else {
                return Err("Invalid input!");
            }
        }

        Ok(Bigint { digits })
    }
}
use std::fmt;
use std::fmt::Write; // опционално, може да ви трябва, може би не, зависи какво правите.
fn filter_digits(digits: &Vec<u8>) -> Vec<u8> {
    let mut numb = Vec::new();
    let mut it = digits.iter();
    let mut temp = it.next();

    while temp== Some(&0) {
        temp=it.next();
    }

    if temp == None {
        numb.push(0);
    }
    else {
        numb.push(*temp.unwrap());
    }
    for val in it {
        numb.push(*val);
    }

    if numb.len() == 0 {
        numb.push(0);
        numb
    }
    else {
        numb
    }
}
fn is_zero(digits: &Vec<u8>) -> bool {
    let vec = digits;
    let vec = filter_digits(vec);
    (vec.len() == 1) && (vec[0] == 0)
}
impl fmt::Display for Bigint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match is_zero(&self.digits) {
            true => write!(f, "{}", 0),
            _ => { 
                let temp = filter_digits(&self.digits);
                 let it = temp.iter();
                for val in it {
                    write!(f, "{}", *val);
                }
                write!(f, "{}", "")
            }
        }
    }
}
pub struct Delimited<'a> {
    bigint: &'a Bigint,
}

impl Bigint {
    pub fn delimited(&self) -> Delimited {
        Delimited { bigint: self }
    }
}

impl<'a> fmt::Display for Delimited<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match is_zero(&self.bigint.digits) {
            true => write!(f, "{}", 0),
            _ => { 
                let temp = filter_digits(&self.bigint.digits);
                let mut counter: i8 = (temp.len() as i8) % 3;
                let mut flag = counter== 0;
                let it = temp.iter();
                for val in it {
                    if flag == true {
                        write!(f, "{}", *val);
                        flag = false;
                    }
                    else if counter % 3 == 0 {
                        write!(f, ",{}", *val);
                    }
                    else {
                        write!(f, "{}", *val);
                    }
                    counter = counter - 1;
                }
                write!(f, "{}", "")
            }
        }
    }
}

fn main() {
    //  let bigint = Bigint::from_str("042900").unwrap();
    //  println!("{}", bigint);
    //  println!("{}", bigint);
    let bigint1 = Bigint::from_str("1").unwrap();
    let bigint2 = Bigint::from_str("12").unwrap();
    let bigint3 = Bigint::from_str("123").unwrap();
    let bigint4 = Bigint::from_str("1234").unwrap();
    let bigint5 = Bigint::from_str("12345").unwrap();
    let bigint6 = Bigint::from_str("123456").unwrap();
    let bigint7 = Bigint::from_str("1234567").unwrap();
    let bigint8 = Bigint::from_str("12345678").unwrap();
    let bigint9 = Bigint::from_str("123456789").unwrap();
    let bigint10 = Bigint::from_str("12345678910").unwrap();
    println!("{}", bigint1.delimited());
    println!("{}", bigint2.delimited());
    println!("{}", bigint3.delimited());
    println!("{}", bigint4.delimited());
    println!("{}", bigint5.delimited());
    println!("{}", bigint6.delimited());
    println!("{}", bigint7.delimited());
    println!("{}", bigint8.delimited());
    println!("{}", bigint9.delimited());
    println!("{}", bigint10.delimited());

    // => 100,000
    //   let bigint = Bigint::from_str("100000000").unwrap();
    //   println!("{}", bigint.delimited());
    //   // => 100,000,000
}
