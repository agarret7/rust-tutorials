use std::collections::HashMap;

#[derive(Debug)]
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vectors() -> Vec<SpreadsheetCell> {
    let v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3, 4, 5];
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    for i in &mut v2 {
        println!("{i}");
        *i += 50;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    row
}

pub fn strings() -> String {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1.clone() + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    let hello = "Здравствуйте";
    let s = String::from(&hello[0..4]);

    s
}

pub fn hash_maps() {
    // defaults to <String, i32>
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}

pub fn median(v: &Vec<i32>) -> Option<i32> {
    let mut v = v.clone();
    v.sort();
    if v.len() == 0 {
        return None;
    } if v.len() % 2 == 0 {
        let x = v.len() / 2;
        let y = x + 1;
        Some((v[x] + v[y]) / 2)
    } else {
        Some(v[v.len() / 2])
    }
}

pub fn mode(v: &Vec<i32>) -> Option<i32> {
    let mut mode: Option<i32> = None;
    let mut mode_count = 0;
    if v.len() == 0 {
        return mode;
    }

    let mut map = HashMap::new();

    for x in v {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    }
    // dbg!(map);

    for (key, value) in &map {
        println!("{key}: {value}");
        match mode {
            None => {
                let mode = Some(key);
                let mode_count = value;
            },
            Some(mode) => {
                if value > &mode_count {
                    let mode = Some(key);
                    let mode_count = value;
                }
            },
        }
    }
    mode
}