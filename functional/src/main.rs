use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn _store() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

fn _closure() {
    let expensive_closure = |num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(1);
}

fn _closure2() {
    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);
    let only_borrows = || println!("From closure {:?}", list);

    println!("Before calling closure {:?}", list);
    only_borrows();
    println!("After calling closure {:?}", list);
}
fn _closure3() {
    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = || list.push(7);
    // println!("{:?}", list);
    borrows_mutably();
    println!("After calling closure {:?}", list);
}

fn _move() {
    let list = vec![1, 2, 3];
    println!("Before defining closure {:?}", list);

    thread::spawn(move || println!("From thread {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn _sort() {
    let mut list = vec![
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

fn _sort2() {
    let mut list = vec![
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {} operations", list, num_sort_operations);
}

fn _iterator() {
    let vec = vec![1, 2, 3];
    let vec_iter = vec.iter();
    for val in vec_iter {
        println!("Got: {}", val);
    }
}

fn _iterator2() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

fn _iterator3() {
    let buffer: &mut [i32] = &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let coefficients: [i64; 12] = [0, -28180, 9744, 6753, 997, -190, 91, -21, 3, -1, 0, 0];
    let qlp_shift: i16 = 10;

    for i in 12..buffer.len() {
        let prediction = coefficients
            .iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>()
            >> qlp_shift;
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}

fn main() {
    _iterator3();
}
