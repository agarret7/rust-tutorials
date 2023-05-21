use  std::fs::File;
use std::io::ErrorKind;


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