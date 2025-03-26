use std::io;

fn main() {
    let x = 5;
    let x = x + 1;
    println!("value of x is: {x}");
    
    {
        let x = x * 2;
        println!("value of x is: {x}");
    }
    println!("value of x is: {x}");

    // let mut spaces = "    ";
    // spaces = spaces.len();

    let spaces = "    ";
    let spaces = spaces.len();

    println!("{spaces}");

    let guess: u32 = "42".parse().expect("not a number!");
    println!("{guess}");

    let sum = 5 + 10;
    println!("{sum}");

    let difference = 95.5 - 4.3;
    println!("{difference}");

    let product = 4 * 30;
    println!("{product}");

    let quotient = 56.7 / 32.2;
    println!("{quotient}");

    let turncated = -5 / 3;
    println!("{turncated}");

    let remainder = 43 % 5;
    println!("{remainder}");

    let t = true;
    println!("{t}");

    let f: bool = false;
    println!("{f}");

    let c = 'z';
    println!("{c}");

    let z: char = 'ℤ';
    println!("{z}");

    let z = '😀';
    println!("{z}");

    let tup: (i32, f64, char) = (500, 6.4, 'x');
    // let tup2 = (500, 6.4, 'x');
    // println!("{tup}");
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");
    // println!("{tup.0}, {tup.1}, {tup.2}");
    println!("tup.0 = {}", tup.0);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let charx = tup.2;
    println!("{five_hundred}, {six_point_four}, {charx}");

    let list_a = [1, 2, 3, 4, 5];
    println!("{}", list_a[0]);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = list_a[index];

    println!("index {index} = {element}");
    println!("==========3-3============");

    another_function(5);
    print_labeled_measurement(10, 'h');

    // let statement = 6;
    // let statement1 = (let statement2 = 6);
    let expression = {
        let statement = 3;
        statement + 1
    };
    println!("value = {expression}");

    let five = five();
    println!("{five}");

    let plus_number = plus_one(five);
    println!("{plus_number}");


    let number = 6;
    
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    if number != 0 {
        println!("0 -> false? no!");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 1 } else { 0 };
    println!("{number}");

    // if condition {
    //     1
    // } else {
    //     0
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("loop result: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        
        count += 1;

    }
    println!("End count = {count}");

    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFEOFF!");

    let a = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < 5 {
        println!("value = {}", a[i]);
        i += 1;
    }

    for element in a {
        println!("in for loop value = {element}");
    }

    for n in (1..4).rev() {
        println!("n = {n}")
    }

}

fn another_function(x: i32) {
    println!("another fn!");
    println!("x = {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("measurement = {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}