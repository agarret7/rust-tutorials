pub fn scope() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let mut received_s = gives_ownership();
    println!("{}", received_s);

    let l = calculate_length(&received_s);
    println!("{}", l);

    change(&mut received_s);

    println!("{}", received_s);

    // let ref_to_null = dangle();
    let mut s = no_dangle();
    let slice = first_word(&s);
    println!("{}", slice);
    s.clear();
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    return some_string;
}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns the length of a String

    return length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello world");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}