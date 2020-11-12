use std::str;

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str{
    if s1.len() > s2.len() {
        s1
    }
    else {
        s2
    }
}

fn first_occurrence<'a, 'b>(s: &'a str, pattern: &'b str) -> Option<&'a str> {
    s.matches(pattern).next()
}

#[derive(Debug)]
struct Words<'a> {
    text: Option<&'a str>,
}

impl<'a> Words<'a> {
    fn new(text: &'a str) -> Self {
        Words { text: Some(text) }
    }
    fn next_word(&mut self) -> Option<&'a str> { 
        let text = self.text?;
        let mut iter = text.splitn(2, char::is_whitespace);

        match (iter.next(), iter.next()) {
            (Some(word), rest) => { self.text = rest; Some(word) },
            _ => unreachable!()
        }
    }
}

fn hello() -> &'static str {
    let mut words = Words::new("hello world");
    words.next_word().unwrap()
}

trait MyTrait {}
impl MyTrait for String {}

struct Wrapper<T: MyTrait>(T);

fn save_for_later<T: MyTrait>(something: T) -> Wrapper<T> {
    Wrapper(something)
}

fn main() {
    let text = String::from("To be or no to be?");
    let result = {
        let pattern = String::from("no");
        first_occurrence(&text, &pattern)
    };
    println!("{:?}", result);

    println!("{}", hello());

    let saved = {
        let s = String::from("Great!");
        save_for_later(s)
    };
    let inner = &saved.0;
    println!("{}", inner);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_from_main() {
        assert_eq!(2 + 2, 4);
    }
}
