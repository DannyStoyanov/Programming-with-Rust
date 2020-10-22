#[derive(Debug)]
enum Token {
    Text(String),
    Number(f64),
}

fn main() {
    // Enums:
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i64, y: i64 },
        Write(String),
        ChangeColor(i64, i64, i64),
    }
    Message::Quit;
    Message::Move { x: 3, y: 4 };
    Message::Write(String::from("baba"));
    Message::ChangeColor(255, 0, 0);

    println!("-------------------");
    // Option enum:
    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("string");
    let absent_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent number: {:?}", absent_number);

    let mut my_numb = Some(9_i32);
    //let mut my_numb: Option<u32> = None;
    match my_numb {
        Some(val) => println!("My number value is: {:?}.", val),
        None => println!("Value not found!"),
    }

    let my_next_numb = match my_numb {
        Some(val) => {
            println!("Successful!");
            Some(val)
        }
        None => {
            println!("Unsuccessful!");
            None
        }
    };
    println!("My next number is: {:?}", my_next_numb);

    println!("-------------------");
    // Pattern Matching:
    let numb_x = 5;
    let numb_y = match numb_x {
        3 => 3,
        5 => 5,
        _ => 0,
    };
    println!("y= {:?}", numb_y);

    println!("-------------------");
    // if let:
    let alpha = Some(8);
    if let Some(8) = alpha {
        println!("Aplha");
    }

    println!("-------------------");
    // while let:
    let array_of_eighties = [8, 8, 8, 88, 8, 888, 8, 8];
    let mut iter8or = array_of_eighties.iter();
    while let Some(8) = iter8or.next() {
        println!("Beta");
    }

    println!("-------------------");
    // iteration:
    let numbers: Vec<&u32> = [1, 2, 3].iter().collect(); // std::slice::Iter
    let chars: Vec<char> = "abc".chars().collect(); // std::str::Chars
    let words: Vec<&str> = "one two three".split_whitespace().collect(); // std::str::SplitWhitespace

    println!("{:?}", numbers);
    println!("{:?}", chars);
    println!("{:?}", words);

    let string = String::from("abc");
    let chars = string.chars();

    println!("{:?}", chars);

    let string = String::from("abc");
    let mut chars = string.chars(); // Mutable!

    println!("{:?}", chars.next());
    println!("{:?}", chars.next());
    println!("{:?}", chars.next());
    println!("{:?}", chars.next());

    println!("-------------------");
    let string = String::from("abc");
    let mut chars = string.chars();

    for c in chars {
        println!("{:?}", c);
    }
    println!("-------------------");
    let counts = [1, 2, 3, 4];
    let mut counter = counts.iter();

    if let Some(n) = counter.next() {
        print!("{}", n);
        while let Some(n) = counter.next() {
            print!(" and {}", n);
        }
        println!();
    }

    println!("-------------------");
    // Pattern Matching Guards:
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Opposite"),
        (x, y) if (x % 2 == 0) & (y % 2 == 0) => println!("Even"),
        (x, y) if (x % 2 != 0) & (y % 2 != 0) => println!("Odd"),
        (x, y) if (x % 2 == 0) & (y % 2 != 0) => println!("x is even, y is odd"),
        (x, y) if (x % 2 != 0) & (y % 2 == 0) => println!("x is odd, y is even"),
        _ => println!("Whatever..."),
    }

    println!("-------------------");
    // Ranges:
    let age: i32 = 21;
    match age {
        n if n < 0 => println!("Not born yet!"),
        0 => println!("Newborn"),
        1...12 => println!("Kid"),
        13...19 => println!("Teen"),
        _ => println!("Adult"),
    }
    println!("-------------------");
    // Multiple patterns
    let score: u32 = 5;

    match score {
        0 | 1 => println!("Low."),
        _ => println!("Good result!"),
    }

    println!("-------------------");
    // Struct:
    struct User {
        name: &'static str,
        age: u8,
    }
    let user = User {
        name: "John",
        age: 23,
    };
    match user {
        User {
            name: "Josh",
            age: _,
        } => println!("Found Josh!"),
        User {
            name: "John",
            age: _,
        } => println!("Hey John!"),
        User { name: _, age: 23 } => println!("Found Josh!"),
        _ => println!(" Uknown!"),
    }

    println!("-------------------");
    // Destructing:
    let token = Token::Text(String::from("Result is 42"));
    match &token {
        Token::Text(text) => println!("The token is text '{}'", text),
        Token::Number(n) => println!("The token is number {}", n),
    }
    println!("At last the token is {:?}!", token);

    println!("-------------------");
    // Refutable and irrefutable pattern:
    let (a, b) = {
        println!("Irrefutable!"); // -> Irrefutable pattern
        (1, 2)
    };

    if let Some(val) = Some(5) {
        println!("Refutable!"); // -> Refutable pattern
    }
}
