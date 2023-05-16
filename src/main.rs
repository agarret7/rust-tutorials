use std::io;

mod chapter1;
mod chapter2;
mod chapter3;
mod chapter4;


fn main() {
    let mut inp_branch = String::new();
    io::stdin()
        .read_line(&mut inp_branch)
        .expect("Failed to read line");
    let branch: i32 = inp_branch
        .trim()
        .parse()
        .expect("Input not an integer");
    
    println!("Executing chapter: {branch}");
    if branch == 1 {
        chapter1::hello_world();
    } else if branch == 2 {
        chapter2::guessing_game();
    } else if branch == 3 {
        let c = chapter3::THREE_HOURS_IN_SECONDS;
        println!("3 hours in seconds: {c}");
        chapter3::variables();
        chapter3::shadowing();
        chapter3::data_types();
        chapter3::array_access();
        chapter3::another_function(5);
        let n: i32 = 10;
        let x = chapter3::fibonacci(n);
        println!("Fibonacci({n}) = {x}")
    } else if branch == 4 {
        chapter4::scope();
    } else {
        println!("Figure out how to make errors");
    }
}