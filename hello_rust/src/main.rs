fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn print(a: i32) -> () {
    println!("variable = {:?}", a);
}

fn main() {
    println!("-----New Execution-----");
    // number types:
    let x: i32 = 5000;
    let y: f32 = 3.14;
    println!("x = {:?}, y = {:?}", x, y);

    // bool:
    let z: bool = true;
    println!("z = {:?}", z);

    // unit:
    let unit: () = ();
    println!("unit value: {:?}", unit);

    // str:
    let name: &str = "Daniel";
    println!("My name is {:?}", name);

    // char:
    let c: char = '\u{2764}';
    println!("c = {:?}", c);

    // array:
    let arr: [i32; 5] = [6, 9, 16, 19, 26];
    println!("arr = {:?}", arr);

    // nested array:
    let matrix: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("matrix = {:?}", matrix);

    // tuples:
    let age: i32 = 21;
    let height: i32 = 186;
    let sex: bool = true; // true for man, false for woman
    let tuple: (i32, i32, bool) = (height, age, sex);
    println!("tuple = {:?}", tuple);

    // if-clause:
    if z {
        println!("z is true");
    } else {
        println!("z is false");
    }

    // for loop:
    let _i: i32 = 0;
    for _i in 0..5 {
        println!("This is a new line!(for)");
    }

    // while loop:
    let mut flag: bool = true;
    let mut counter: i32 = 0;
    while flag {
        if counter == 5 {
            flag = false;
        }
        counter += 1;
        println!("This is a new line!(while)");
    }

    // add function:
    println!("add(5, 6) = {:?}", add(5, 6));

    // print function:
    print(45);
}
