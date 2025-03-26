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

    println!("{s2} length = {len}")

}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
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
