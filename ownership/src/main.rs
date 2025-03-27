fn main() {
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{s1} world {s2}");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    let s = String::from("hello");

    takes_ownership(s.clone());
    println!("after takes_ownership function s={s}");

    let x = 5;

    makes_copy(x); // i32 is Copy so keep use x
    println!("after makes_copy function x={x}");

    let s1 = gives_ownership();
    let s2 = String::from("s2 string");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {s1}, s2: drop, s3: {s3}");

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("{s2} length = {len}");

    let s1 = String::from("hello");
    let len = calculate_length_reference(&s1);

    println!("s1={s1} len={len}");

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    // {
    //     let r1 = &mut s;
    // }
    
    // let r2 = &mut s;

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}");

    let r3 = &mut s;

    println!("{r3}");

    let reference_to_nothing = dangle();

    println!("{reference_to_nothing}")

}

fn dangle() -> String {
    let s = String::from("hello");
    // &s
    s
}

fn change(some_str: &mut String) {
    some_str.push_str(", world");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_reference(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string drop -> free

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // some_integer is copied

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
