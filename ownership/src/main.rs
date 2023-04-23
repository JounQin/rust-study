fn _string() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn make_copy(i: i32) {
    println!("{}", i);
}

fn _copy() {
    let s = String::from("Hello");
    takes_ownership(s);
    let x = 5;
    make_copy(x);
    println!("{}", x);
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn _return() {
    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn _tuple() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length2(s: &String) -> usize {
    // s.push_str(", world");
    s.len()
}

fn _reference() {
    let s1 = String::from("hello");
    let len = calculate_length2(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn _mut() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
    change(&mut s);
    println!("{}", s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn _word() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();
    println!("{}", word);
}

fn _slice() {
    let s = String::from("Hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{}, {}", hello, world);
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn _word2() {
    let mut s = String::from("hello world");
    let word = first_word2(&s);
    println!("{}", word);
    s.clear();
    println!("{}", s);
    // println!("{}", word);
}

fn _arr_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..=3];
    assert_eq!(slice, &[2, 3, 4]);
}

fn main() {
    _arr_slice();
}
