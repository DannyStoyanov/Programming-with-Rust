pub fn fizzbuzz(n: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if n <= 0 {
        return result;
    }

    for _i in 1..(n + 1) {
        if (_i % 3 == 0) & (_i % 5 != 0) {
            result.push(String::from("Fizz"));
        } else if (_i % 5 == 0) & (_i % 3 != 0) {
            result.push(String::from("Buzz"));
        } else if (_i % 3 == 0) & (_i % 5 == 0) {
            result.push(String::from("Fizzbuzz"));
        } else {
            result.push(_i.to_string());
        }
    }

    result
}
pub fn custom_buzz(n: usize, k1: u8, k2: u8) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    if n <= 0 {
        return result;
    }
    if (k1 <= 1) || (k2 <= 1) {
        panic!("Bad divisors!");
    }

    for _i in 1..(n + 1) {
        let temp = _i as u8;
        if (temp % k1 == 0) & (temp % k2 != 0) {
            result.push(String::from("Fizz"));
        } else if (temp % k2 == 0) & (temp % k1 != 0) {
            result.push(String::from("Buzz"));
        } else if (temp % k1 == 0) & (temp % k2 == 0) {
            result.push(String::from("Fizzbuzz"));
        } else {
            result.push(temp.to_string());
        }
    }

    result
}
pub struct FizzBuzzer {
    pub k1: u8,
    pub k2: u8,
    pub labels: [String; 3],
}
impl FizzBuzzer {
    pub fn take(&self, n: usize) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        if n <= 0 {
            return result;
        }
        if (self.k1 <= 1) || (self.k2 <= 1) {
            panic!("Bad divisors!");
        }
        for _i in 1..(n + 1) {
            let temp = _i as u8;
            if (temp % self.k1 == 0) & (temp % self.k2 != 0) {
                result.push(self.labels[0].to_string());
            } else if (temp % self.k2 == 0) & (temp % self.k1 != 0) {
                result.push(String::from(self.labels[1].to_string()));
            } else if (temp % self.k1 == 0) & (temp % self.k2 == 0) {
                result.push(String::from(self.labels[2].to_string()));
            } else {
                result.push(temp.to_string());
            }
        }
        result
    }
    pub fn change_label(&mut self, index: usize, value: &String) {
        self.labels[index] = value.to_string();
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
