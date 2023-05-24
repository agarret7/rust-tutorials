// Code is mostly copied from "the book": https://doc.rust-lang.org/stable/book/title-page.html
// Thanks to Steve Klabnik and Carol Nichols, and the Rust Community for a very enjoyable learning experience

use std::{env,io,process, collections::HashMap};


mod chapter1; mod chapter2; mod chapter3; mod chapter4; mod chapter5; mod chapter6;
mod chapter8; mod chapter9; mod chapter10;

pub mod garden; pub mod aggregator; pub mod minigrep;

use crate::garden::vegetables::Asparagus;
use aggregator::{Summary, Tweet, NewsArticle};


fn main() -> Result<(), String> {
    let mut inp_branch = String::new();
    println!("Input chapter to run: ");
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
        let msg = chapter6::Message::Write(String::from("goodbye"));
        chapter6::route(&ip, msg);
        let msg = chapter6::Message::Quit;
        chapter6::route(&ip, msg);
        chapter6::value_in_cents(chapter6::Coin::Quarter(chapter6::UsState::Arizona));
    } else if branch == 7 {
        let plant = Asparagus {};
        println!("I'm growing {:?}!", plant);
        rust_tutorials::customer::eat_at_restaurant();
    } else if branch == 8 {
        let row = chapter8::vectors();
        println!("row: {:?}", row);
        let str = chapter8::strings();
        let hmap = chapter8::hash_maps();
        for c in str.chars() {
            println!("{c}");
        }
        for b in str.bytes() {
            println!("{b}");
        }

        // Exercise 1
        // Given a list of integers, use a vector and return the median
        // (when sorted, the value in the middle position) and mode
        // (the value that occurs most often; a hash map will be helpful here) of the list.
        let v = vec![1, 5, 10, 2, 15];
        let median = chapter8::median(&v);
        println!("median: {}", median.unwrap());
        let v = vec![1, 5, 6, 6, 2, 8, 3, 7];
        let median = chapter8::median(&v);
        println!("median: {}", median.unwrap());
        let mode = chapter8::mode(&v);
        println!("mode: {}", mode.unwrap());

        let v = vec![];
        let mode = chapter8::mode(&v);

        // this would panic, so we comment it out.
        // println!("mode: {}:", mode.unwrap());

        // Exercise 2
        // Convert strings to pig latin. The first consonant of each word is moved
        // to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
        // Words that start with a vowel have “hay” added to the end instead (“apple”
        // becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
        let s = "first";
        let pl = chapter8::to_pig_latin(&s);
        println!("{}", pl);
        let s = "apple";
        let pl = chapter8::to_pig_latin(&s);
        println!("{}", pl);

        // Exercise 3
        // Using a hash map and vectors, create a text interface to allow a user
        // to add employee names to a department in a company. For example,
        // “Add Sally to Engineering” or “Add Amir to Sales”. Then let the user
        // retrieve a list of all people in a department or all people
        // in the company by department, sorted alphabetically.
        let mut company: HashMap<String,Vec<String>> = HashMap::new();
        chapter8::hiring_manager(&mut company);
        chapter8::sort_departments(&mut company);
        for (department, members) in &company {
            println!("HackerCo");
            println!("--------");
            print!("{}: ", department);
            for person in members {
                print!("{}, ", person);
            }
            print!("\n");
        }
    } else if branch == 9 {
        // chapter9::one();
        // chapter9::two();
        chapter9::three();
        chapter9::four();
        // todo: come back and write username "agarrett" to file
        let my_username = chapter9::read_username_from_file().unwrap();
        let c = chapter9::last_char_of_first_line("e-ai-e-ai-o");
        let my_placeholder_ip = chapter9::placeholder_ip();
        println!("player: {}", my_username);
        println!("placeholder ip: {}", my_placeholder_ip);
    } else if branch == 10 {
        chapter10::main();
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());

        chapter10::main2();
    } else if branch == 11 {
        println!("run tests with 'cargo test'");
    } else if branch == 12 {
        let args: Vec<String> = env::args().collect();

        // this is called a "closure"
        let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

        println!("Searching for {}", config.query);
        println!("In file {}", config.file_path);
        
        if let Err(e) = minigrep::run(config) {
            println!("Application error: {e}");
            process::exit(1);
        }
    } else if 1 <= branch && branch <= 20 {
        println!("Unimplemented chapter: {}. Exiting.", branch);
    } else {
        return Err(format!("Unknown chapter: {}", branch));
    }
    Ok(())
}