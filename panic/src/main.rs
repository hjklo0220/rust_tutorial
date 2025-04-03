use std::error::Error;
use std::fs::File;
use std::net::IpAddr;

use crate::result::recoverable;
use crate::result::shortcut;
use crate::result::propagating;

pub mod result;

fn main() -> Result<(), Box<dyn Error>>{
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    recoverable::open_file();
    println!("{:?}", shortcut::shortcut());
    println!("{:?}", propagating::read_username_from_file());
    println!("very short: {:?}", shortcut::very_short());
    println!("very very short: {:?}", shortcut::orignal());

    let greeting_file = File::open("hello.txt")?;

    let text = String::from("\nhi");
    println!("{:?}", last_char_of_first_line(&text));

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("hardcoded IP address should be vaild");

    println!("{home}");

    Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}