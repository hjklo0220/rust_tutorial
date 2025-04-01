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

    let mut s = String::new();
    let data = "inital contents";

    let s = data.to_string();

    let s = "inital contents".to_string();
    println!("{s}");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("test");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2={s2}");
    s1.push('a');
    println!("{s1}");

    let s1 = String::from("hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let s1 = String::from("hello");
    // let h = s1[0];
    
    let hello = "안녕하세요";
    let s = &hello[0..6];
    // println!("{s}");

    for c in "폭싹속았쑤다".chars() {
        println!("{c}");
    }

    for b in "러스트".bytes() {
        println!("{b}");
    }

}