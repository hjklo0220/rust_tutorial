use std::{fs::{self, File}, io::{self, Read}};

pub fn shortcut()  -> Result<String, io::Error>{
    // let greeting_file = File::open("world.txt").unwrap();

    // let another_file = File::open("test.txt")
    //     .expect("test.txt should be included in this project");

    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn very_short() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

pub fn orignal()-> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}