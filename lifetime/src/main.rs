use std::fmt::Display;

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest2<'a>(x: &str, y: &str) -> &'a str {
    todo!();
    // String::from("really long string").as_str()
}

fn _longest() {
    let string1 = String::from("abcd");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn _lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if &item == &b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn _lifetime2() {
    let s: &'static str = "I have a static lifetime.";
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", announcement);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    _lifetime()
}
