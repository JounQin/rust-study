use std::collections::HashMap;

fn _vector1() {
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);
    let v = vec![1, 2, 3];
    println!("v: {:?}", v);
}

fn _vector2() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v: {:?}", v);
}

fn _vector3() {
    let v = vec![1, 2, 3, 4, 5];
    let third = v[2];
    println!("third: {}", third);
    let third = v.get(2);
    match third {
        Some(third) => println!("third: {}", third),
        _ => (),
    }
}

fn _vector4() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6);
    println!("first: {}", first);
}

fn _vector5() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("i: {}", i);
    }
}
fn _vector6() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("v: {:?}", v);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

impl SpreadsheetCell {
    fn value(&self) -> String {
        match self {
            SpreadsheetCell::Int(i) => i.to_string(),
            SpreadsheetCell::Float(f) => f.to_string(),
            SpreadsheetCell::Text(t) => t.to_string(),
        }
    }
}

fn _vector7() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{}", row[2].value())
}

fn _str1() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('!');
    println!("s1: {}, s2: {}", s1, s2);
}

fn _str2() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3: {}", s3)
}

fn _str3() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");
}

fn _str4() {
    let hello = "Здравствуйте";
    println!("len: {}", hello.len());

    for char in hello.chars() {
        println!("char: {}", char);
    }
}

fn _map1() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get("Blue").copied().unwrap_or(0);
    println!("score: {}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn _map2() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // println!("{}: {}", field_name, field_value)
}

fn _map3() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("scores: {:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores: {:?}", scores);
}

fn _map4() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}

fn main() {
    _map4();
}
