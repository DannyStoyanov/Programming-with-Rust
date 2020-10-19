fn takes_ownership(some_string: String) {
    println!("{:?}", some_string);
} // some_string out of scope and releasing the memory

fn makes_copy(some_integer: i32) {
    println!("{:?}", some_integer);
} // some_integer out of scope, but nothing fabulous happens

fn gives_ownership() -> String {
    let some_string = String::from("More cookies!");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/*
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
*/

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) -> () {
    some_string.push_str(", world!");
}
fn main() {
    // Move semantics:
    /*
    let s1 = String::from("Cookies!");
    let s2 = s1.clone();

    println!("{:?}", s2);
    takes_ownership(s1); // s1's value moved to the function
    */

    // println!("{:?}", s1); // would be error because s1 dont have ownership to the string "Cookies!"

    // but:
    let x = 5;
    makes_copy(x);
    println!("{:?}", x); // this is possible because i32 is Copy

    // returned value from function gives ownership:
    /*
    let s1 = gives_ownership();
    let s2 = String::from("More cookies!");
    let s3 = takes_and_gives_back(s2);
    */

    /*
    let s1 = String::from("Cookies!");
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    */

    // using value without move semantics:
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // immutable:
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    // Slices:
    let mut s = String::from("Hello, world!");
    let r1 = &s[..];
    let r2 = &r1[0..4];

    println!("{}", r2);

    // Vectors:
    let mut v = Vec::new();
    v.push(1);
    v.push(4);
    v.push(6);
    v.push(9);
    println!("{:?}", v);

    let v1 = vec![1, 2, 4, 7];
    println!("{:?}", v1);

    // Slice in vec:
    let arr = [1, 2, 3];
    let slice = &arr[..];
    println!("{:?}", slice);

}
