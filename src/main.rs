use std::io;

mod chapter1;
mod chapter2;


fn main() {
    let mut inp_branch = String::new();
    io::stdin()
        .read_line(&mut inp_branch)
        .expect("Failed to read line");
    let branch: i32 = inp_branch
        .trim()
        .parse()
        .expect("Input not an integer");
    
    println!("Executing tutorial: {branch}");
    if branch == 1 {
        chapter1::hello_world()
    } else if branch == 2 {
        chapter2::guessing_game()
    } else {
        println!("Figure out how to make errors");
    }
}