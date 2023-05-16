use std::io;


pub const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn variables() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

pub fn shadowing() {
    let x: i32 = 5;
    let x = x + 1;

    {
        let x = x.pow(2);
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x in the outer scope is: {x}");
}

pub fn data_types() {
    let _guess: u32 = "42".parse().expect("Not a number!");

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
    let z = _x + _y;
    println!("The value of z = x + y = {z}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, z) = tup;

    println!("The shadowed value of z = {z}");
}

pub fn array_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

pub fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

// skipped rest

pub fn fibonacci(n : i32) -> i32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}