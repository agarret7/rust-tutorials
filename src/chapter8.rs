use std::{io, collections::HashMap};

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

    for (&key, &value) in &map {
        println!("{key}: {value}");
        match mode {
            None => {
                mode = Some(*key);
                mode_count = value;
            },
            Some(_) => {
                if value > mode_count {
                    mode = Some(*key);
                    mode_count = value;
                }
            },
        }
    }
    mode
}

const VOWELS: [char; 10] =
    ['a', 'e', 'i', 'o', 'u',
     'A', 'E', 'I', 'O', 'U'];

pub fn to_pig_latin(s: &str) -> String {
    if s.len() == 0 { return s.to_string(); }
    let first = &s[0..1];
    let s = &s[1..];
    if VOWELS.contains(&first.chars().next().expect("should never get here")) {
        return format!("{first}{s}-hay");
    } else {
        return format!("{s}-{first}ay");
    }
}


pub fn hiring_manager(company: &mut HashMap<String,Vec<String>>) -> &mut HashMap<String,Vec<String>> {
    loop {
        let mut instruction = String::new();

        io::stdin()
            .read_line(&mut instruction)
            .expect("Failed to read line");

        let instruction: String = match instruction
            .trim()
            .parse() {
                Ok(s) => s,
                Err(_) => continue,
        };

        if instruction.len() == 0 {
            break;
        }

        let mut words = instruction.split(" ");
        assert_eq!(words.next().expect("expected \"Add\""), "Add");
        let person = &words.next().expect("expected person");
        let person = person.to_string();
        assert_eq!(words.next().expect("expected \"to\""), "to");
        let department = &words.next().expect("expected department");
        let department = department.to_string();

        let members = company.entry(department).or_insert(Vec::new());
        members.push(person);
    }
    company
}

pub fn sort_departments(company: &mut HashMap<String,Vec<String>>) -> &mut HashMap<String,Vec<String>> {
    for members in company.values_mut() {
        members.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    }
    company
}