use  std::fs::File;
use std::io::{self, Write, Read, ErrorKind};
use std::net::IpAddr;


pub fn one() {
    panic!("crash and burn");
}

pub fn two() {
    let v = vec![1, 2, 3];
    v[99];
}

pub fn three() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };
}

pub fn four() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    // more verbose version of the code at the bottom
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // even shorter
    // fs::read_to_string("hello.txt")
}

pub fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn placeholder_ip() -> IpAddr {
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    home
}