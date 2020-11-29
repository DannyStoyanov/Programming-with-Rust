use std::char;
use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::str::FromStr;

fn is_valid_sign(sign: &i8) -> bool {
    (*sign == -1) || (*sign == 1)
}
fn filter_digits(digits: &Vec<u8>) -> Vec<u8> {
    let mut numb = Vec::new();
    let mut it = digits.iter();
    let mut temp = it.next();

    while temp == Some(&0) {
        temp = it.next();
    }

    if temp == None {
        numb.push(0);
    } else {
        numb.push(*temp.unwrap());
    }
    for val in it {
        numb.push(*val);
    }

    if numb.len() == 0 {
        numb.push(0);
        numb
    } else {
        numb
    }
}
fn is_zero(digits: &Vec<u8>) -> bool {
    let vec = digits;
    let vec = filter_digits(vec);
    (vec.len() == 1) && (vec[0] == 0)
}
fn is_sign(symbol: &char) -> bool {
    *symbol == '+' || *symbol == '-'
}
fn char_to_u8(symbol: &char) -> u8 {
    (*symbol as u8) - ('0' as u8)
}
fn is_sign_first(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for elem in chars {
        if is_sign(&elem) {
            return true;
        }
        if elem != ' ' {
            return false;
        }
    }
    true
}
fn is_valid_string(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut sign: u8 = 0;
    for elem in chars {
        if is_sign(&elem) {
            sign = sign + 1;
        } else if !elem.is_digit(10) && !(elem == ' ') {
            return false;
        }
    }
    if sign > 1 {
        false
    } else if sign == 1 {
        is_sign_first(s)
    } else {
        true
    }
}
fn clear_spaces(digits: &Vec<char>) -> Vec<u8> {
    let mut numb = Vec::new();
    let it = digits.iter();

    for val in it {
        if *val != ' ' && !is_sign(&(*val)) {
            numb.push(char_to_u8(&(*val)));
        }
    }
    numb
}
fn match_sign(symbol: &char) -> i8 {
    match *symbol {
        '+' => 1,
        '-' => -1,
        _ => 1,
    }
}
fn get_sign(s: &str) -> i8 {
    let chars: Vec<char> = s.chars().collect();
    for elem in chars {
        if is_sign(&elem) {
            return match_sign(&elem);
        }
    }
    1
}
#[derive(Debug, PartialEq, Eq)]
pub struct Bigint {
    sign: i8,
    digits: Vec<u8>,
}

impl Bigint {
    pub fn new() -> Self {
        Bigint {
            sign: 1,
            digits: vec![0],
        }
    }

    fn from_components(digits: Vec<u8>, sign: i8) -> Self {
        if is_zero(&digits) {
            Bigint {
                sign: 1,
                digits: filter_digits(&digits),
            }
        } else if is_valid_sign(&sign) {
            Bigint {
                sign: sign,
                digits: filter_digits(&digits),
            }
        } else {
            panic!("Incorrect input for Bigint!");
            //Bigint::new()
        }
    }

    pub fn is_positive(&self) -> bool {
        self.sign == 1
    }

    pub fn is_negative(&self) -> bool {
        self.sign == -1
    }
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for Bigint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_string(&s) {
            let sign: i8 = get_sign(&s);
            let chars: Vec<char> = s.chars().collect();
            let numb: Vec<u8> = clear_spaces(&chars);
            Ok(Bigint::from_components(numb, sign))
        } else {
            Err(ParseError)
        }
    }
}

fn bigint(s: &str) -> Bigint {
    Bigint::from_str(s).unwrap()
}

fn are_both_zero(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    let expr1 = a.len() == 1 && a[0] == 0;
    let expr2 = b.len() == 1 && b[0] == 0;
    expr1 && expr2
}
fn direct_compare(a: &Vec<u8>, b: &Vec<u8>) -> i8 {
    if a.len() > b.len() {
        return 1;
    } else if a.len() < b.len() {
        return -1;
    } else {
        for i in 0..a.len() {
            if a[i] > b[i] {
                return 1;
            } else if a[i] < b[i] {
                return -1;
            }
        }
    }
    0
}
impl PartialOrd for Bigint {
    fn partial_cmp(&self, other: &Bigint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bigint {
    fn cmp(&self, other: &Bigint) -> Ordering {
        match self.sign.cmp(&other.sign) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => match self.digits.len().cmp(&other.digits.len()) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => {
                    if are_both_zero(&self.digits, &other.digits) {
                        return Ordering::Equal;
                    } else {
                        if direct_compare(&self.digits, &other.digits) == 1 {
                            return Ordering::Greater;
                        } else if direct_compare(&self.digits, &other.digits) == -1 {
                            return Ordering::Less;
                        } else {
                            return Ordering::Equal;
                        }
                    }
                }
            },
        }
    }
}

fn max(a: &usize, b: &usize) -> usize {
    if *a > *b {
        return *a;
    } else {
        return *b;
    }
}

fn bigger_vec<'a>(x: &'a Vec<u8>, y: &'a Vec<u8>) -> &'a Vec<u8> {
    if x.len() > y.len() {
        return x;
    } else if x.len() < y.len() {
        return y;
    } else {
        match direct_compare(&x, &y) {
            -1 => return y,
            1 => return x,
            0 => return x,
            _ => return x,
        }
    }
}
fn smaller_vec<'a>(x: &'a Vec<u8>, y: &'a Vec<u8>) -> &'a Vec<u8> {
    if x.len() < y.len() {
        return x;
    } else if x.len() > y.len() {
        return y;
    } else {
        match direct_compare(&x, &y) {
            -1 => return y,
            1 => return x,
            0 => return y,
            _ => return y,
        }
    }
}
fn one_on_mind(a: &mut Vec<u8>) -> Vec<u8> {
    let mut numb: Vec<u8> = Vec::with_capacity(a.len() + 1);
    let mut temp = 0;
    for i in 0..a.len() {
        numb.push((a[i] + temp) % 10);
        temp = (a[i] + temp) / 10;
    }
    if temp != 0 {
        numb.push(temp);
    }
    numb.reverse();
    numb
}
fn expand_vec(a: &Vec<u8>, len: &usize) -> Vec<u8> {
    let mut numb = a.clone();
    numb.reverse();
    while numb.len() < *len {
        numb.push(0);
    }
    numb.reverse();
    numb
}
fn add_vectors(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut bigger = bigger_vec(&a, &b).clone();
    let mut smaller = expand_vec(smaller_vec(&a, &b), &bigger.len());
    if a.len() == b.len() {
        bigger = a.clone();
        smaller = b.clone();
    }
    bigger.reverse();
    smaller.reverse();
    let mut numb = bigger.clone();
    for i in 0..smaller.len() {
        numb[i] = numb[i] + smaller[i];
    }
    let res = one_on_mind(&mut numb);
    res
}
fn sub_vectors(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    // consider a > b
    let mut bigger = a.clone();
    let mut smaller = expand_vec(&b.clone(), &a.len());
    bigger.reverse();
    smaller.reverse();
    let mut numb = bigger.clone();
    let mut temp: i8 = 0;
    for i in 0..smaller.len() {
        let main = numb[i] as i8;
        let secondary = smaller[i] as i8;

        if (main - temp) - secondary < 0 {
            numb[i] = ((main - temp) + 10 - secondary) as u8;
            temp = 1;
        } else {
            numb[i] = ((main - temp) - secondary) as u8;
            temp = 0;
        }
    }
    numb.reverse();
    numb
}
impl Add for Bigint {
    type Output = Bigint;
    fn add(self, other: Self) -> Self {
        if self.sign == other.sign {
            return Bigint {
                sign: self.sign,
                digits: add_vectors(&self.digits, &other.digits),
            };
        } else {
            match direct_compare(&self.digits, &other.digits) {
                1 => {
                    return Bigint {
                        sign: self.sign,
                        digits: filter_digits(&sub_vectors(&self.digits, &other.digits)),
                    }
                }
                -1 => {
                    return Bigint {
                        sign: other.sign,
                        digits: filter_digits(&sub_vectors(&other.digits, &self.digits)),
                    }
                }
                0 => {
                    return Bigint {
                        sign: 1,
                        digits: vec![0],
                    }
                }
                _ => {
                    return Bigint {
                        sign: 1,
                        digits: vec![0],
                    }
                }
            }
        }
    }
}

impl Sub for Bigint {
    type Output = Bigint;
    fn sub(self, other: Self) -> Self {
        let res_sign = match other.sign {
            1 => -1,
            -1 => 1,
            _ => 1,
        };
        let temp_number = Bigint {
            sign: res_sign,
            digits: other.digits,
        };
        if self.sign == temp_number.sign {
            return Bigint {
                sign: self.sign,
                digits: add_vectors(&self.digits, &temp_number.digits),
            };
        } else {
            match direct_compare(&self.digits, &temp_number.digits) {
                1 => {
                    return Bigint {
                        sign: self.sign,
                        digits: filter_digits(&sub_vectors(&self.digits, &temp_number.digits)),
                    }
                }
                -1 => {
                    return Bigint {
                        sign: temp_number.sign,
                        digits: filter_digits(&sub_vectors(&temp_number.digits, &self.digits)),
                    }
                }
                0 => {
                    return Bigint {
                        sign: 1,
                        digits: vec![0],
                    }
                }
                _ => {
                    return Bigint {
                        sign: 1,
                        digits: vec![0],
                    }
                }
            }
        }
    }
}
// --------------------------TESTS-------------------------- //
#[test]
fn test_is_valid_string() {
    // base cases:
    assert_eq!(is_valid_string(""), true);
    assert_eq!(is_valid_string(" "), true);
    assert_eq!(is_valid_string("+"), true);
    assert_eq!(is_valid_string("-"), true);
    assert_eq!(is_valid_string("+0"), true);
    assert_eq!(is_valid_string("-0"), true);
    assert_eq!(is_valid_string("0"), true);
    assert_eq!(is_valid_string("0+"), false);
    assert_eq!(is_valid_string("++"), false);

    // true:
    assert_eq!(is_valid_string("+                   28"), true);
    assert_eq!(is_valid_string("   -          16"), true);
    assert_eq!(is_valid_string("4    9 9 9 "), true);
    assert_eq!(is_valid_string("   784"), true);
    assert_eq!(is_valid_string(" - 32  4"), true);

    // false:
    assert_eq!(is_valid_string("  a   8 2 1"), false);
    assert_eq!(is_valid_string("9 -"), false);
    assert_eq!(is_valid_string("  a  + 234"), false);
    assert_eq!(is_valid_string(" 23+4 9 9 9 "), false);
    assert_eq!(is_valid_string("  -  + 234"), false);
}
#[test]
fn test_from_Str() {
    assert_eq!(Bigint::new(), bigint("0"));
    assert!(Bigint::from_str("foobar").is_err());
    assert_eq!(Bigint::from_str("+0").unwrap(), bigint("-0"));
    assert!(Bigint::from_str("-- 9 9 99 9 ").is_err());
    assert!(Bigint::from_str("a").is_err());
    assert!(Bigint::from_str("+0-").is_err());
    assert!(Bigint::from_str("-0              1+").is_err());
}
#[test]
fn test_sign() {
    // positive:
    assert!(bigint("1").is_positive());
    assert!(bigint("+  00232313221").is_positive());

    // negative:
    assert!(bigint("-1").is_negative());
    assert!(bigint("-12132131").is_negative());
    assert!(bigint("-02130").is_negative());
}

#[test]
fn test_ord() {
    let result = bigint("+123").cmp(&bigint("+123"));
    assert_eq!(Ordering::Equal, result);

    let result = bigint("-123").cmp(&bigint("-123"));
    assert_eq!(Ordering::Equal, result);

    let result = bigint("123").cmp(&bigint("123"));
    assert_eq!(Ordering::Equal, result);

    let result = bigint("+123").cmp(&bigint("123"));
    assert_eq!(Ordering::Equal, result);

    let result = bigint("123").cmp(&bigint("-123"));
    assert_eq!(Ordering::Greater, result);
    let result = bigint("123").cmp(&bigint("1223"));
    assert_eq!(Ordering::Less, result);
}

#[test]
fn test_add() {
    assert_eq!(bigint("123") + bigint("456"), bigint("579"));
    assert_eq!(bigint("-1000") + bigint("-100"), bigint("-1100"));
    assert_eq!(bigint("0") + bigint("0"), bigint("0"));
    assert_eq!(bigint("0") + bigint("0"), bigint(""));
    assert_eq!(bigint("100") + bigint("100"), bigint("200"));

    assert_eq!(bigint("-123") + bigint("456"), bigint("333"));
    assert_eq!(bigint("-1000") + bigint("+100"), bigint("-900"));
    assert_eq!(bigint("12") + bigint("-9"), bigint("3"));
    assert_eq!(bigint("-12") + bigint("-24"), bigint("-36"));
}

#[test]
fn test_sub() {
    assert_eq!(bigint("0") - bigint("0"), bigint("0"));
    assert_eq!(bigint("-1000") - bigint("-100"), bigint("-900"));
    assert_eq!(bigint("0") - bigint("-0"), bigint(""));
    assert_eq!(bigint("100") - bigint("100"), bigint("000"));

    assert_eq!(bigint("-123") - bigint("456"), bigint("-579"));
    assert_eq!(bigint("-1000") - bigint("+100"), bigint("-1100"));
    assert_eq!(bigint("12") - bigint("-9"), bigint("21"));
    assert_eq!(bigint("-12") - bigint("-24"), bigint("12"));
}
