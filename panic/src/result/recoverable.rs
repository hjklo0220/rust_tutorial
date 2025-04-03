use std::fs::File;
use std::io::ErrorKind;

pub fn open_file() {
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opning the file: {:?}", error),
    // };

    // let greeting_file = matc
    
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("{:?}", greeting_file);

}