extern crate core;

use num::BigUint;
use num::traits::{Zero, One};

fn _plus_one(x: i32) -> i32 {
    x + 1
}

fn _loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("The count = {count}");
}

fn _while() {
    let mut number = 3;
    while number > 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn _while_index() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn _for() {
    let a = [10, 20, 30, 40, 50];
    for it in a {
        println!("the value is: {}", it);
    }
}

fn _for_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn fibonacci(n: u32) {
    let mut i = BigUint::zero();
    let mut j = BigUint::zero();
    for m in 0..n + 1 {
        if j.is_zero() {
            j = BigUint::one();
        } else {
            let k = i + j.clone();
            i = j;
            j = k;
        }
        println!("fibonacci({}) = {}", m, i);
    }
}

fn main() {
    fibonacci(1000);
}
