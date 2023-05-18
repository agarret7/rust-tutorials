use std::io;

mod chapter1; mod chapter2; mod chapter3; mod chapter4; mod chapter5;
mod chapter6;


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
    } else if branch == 5 {
        let (_u1, _u2, _black, _origin) = chapter5::ready_up();
    } else if branch == 6 {
        let ip = chapter6::web_ready();
        let msg = chapter6::Message::Write(String::from("hello"));
        chapter6::route(&ip, msg);
        let msg = chapter6::Message::Quit;
        chapter6::route(&ip, msg);
        chapter6::value_in_cents(chapter6::Coin::Quarter(chapter6::UsState::Arizona));
    } else {
        println!("Figure out how to make errors");
    }
}