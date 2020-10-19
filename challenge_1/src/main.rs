// nth fibonacci number:
pub fn fib(n: u32) -> u32 {
    let mut first: u32 = 1;
    let mut second: u32 = 1;
    // not checking n < 0 because n is u32
    if n == 0 || n == 1 {
        return first;
    } else {
        for _i in 1..n {
            let temp: u32 = second;
            second += first;
            first = temp;
        }
    }
    second
}
fn main() {
    //println!("{}", fib(5));
}