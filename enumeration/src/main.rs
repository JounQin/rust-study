enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn _enum1() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

fn _enum2() {
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn _enum3() {
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
}

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn _message() {
    let m = Message::Write(String::from("hello"));
    m.call()
}

fn _option() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(c: Coin2) -> u8 {
    match c {
        Coin2::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(o: Option<i32>) -> Option<i32> {
    match o {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn _option2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn _if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("max is {}", max);
    }

    let coin = Coin2::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin2::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn main() {
    _if_let()
}
