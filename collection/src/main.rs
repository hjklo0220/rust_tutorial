use std::fmt::format;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.pop();

    let v = vec![1, 2, 3, 4, 5];
    
    let third: &i32 = &v[2];
    println!("thrid = {third}");

    let thrid: Option<&i32> = v.get(2);
    match thrid {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let v = vec![1, 2, 3];
    // let does_not_exist = &v[100]; // -> panic!
    let does_not_exist = v.get(100); // -> None

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &mut v[0];
    // v.push(6); // borrowed

    println!("The first element is: {first}");

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

}