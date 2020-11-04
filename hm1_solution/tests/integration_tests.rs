use hm1_solution::*;
#[test]

fn test_basic() {
    let expected = vec![1.to_string(), 2.to_string(), String::from("Fizz")];

    assert_eq!(fizzbuzz(3), expected);
    assert_eq!(custom_buzz(3, 3, 5), expected);

    let mut fizzbuzzer = FizzBuzzer {
        k1: 3,
        k2: 5,
        labels: [
            String::from("Fizz"),
            String::from("Buzz"),
            String::from("Fizzbuzz"),
        ],
    };
    assert_eq!(fizzbuzzer.take(3), expected);
    fizzbuzzer.change_label(0, &String::from("Fiz"));
}
#[test]
fn test_change_labels() {
    let expected = vec![
        1.to_string(),
        2.to_string(),
        String::from("Fizz"),
        4.to_string(),
        String::from("Buzz"),
    ];

    assert_eq!(fizzbuzz(5), expected);
    assert_eq!(custom_buzz(5, 3, 5), expected);

    let mut obj = FizzBuzzer {
        k1: 3,
        k2: 5,
        labels: [
            String::from("ПФОЗ"),
            String::from("ПФОЕС"),
            String::from("ПФОЗИЕС"),
        ],
    };
    let expected = vec![
        1.to_string(),
        2.to_string(),
        String::from("ПФОЗ"),
        4.to_string(),
        String::from("ПФОЕС"),
    ];

    assert_eq!(obj.take(5), expected);

    let expected = vec![
        1.to_string(),
        2.to_string(),
        String::from("Fiz"),
        4.to_string(),
        String::from("ПФОЕС"),
    ];
    obj.change_label(0, &String::from("Fiz"));
    assert_eq!(obj.take(5), expected);
}
