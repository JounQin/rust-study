use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref};
use std::slice;
use advanced::my_vec;

fn _row_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        *r2 -= 1;

        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r is: {}", *r);
    }
}

unsafe fn dangerous() {}

fn _unsafe() {
    unsafe {
        dangerous();
    }
}

fn _unsafe2() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v;
    let (a, b) = r.split_at_mut(3);
    assert_eq!(&mut [1, 2, 3], a);
    assert_eq!(&mut [4, 5, 6], b);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);

    // (&mut values[..mid], &mut values[mid..])

    let prt = values.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(prt, mid),
            slice::from_raw_parts_mut(prt.add(mid), len - mid),
        )
    }
}

fn _unsafe3() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v;
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(&mut [1, 2, 3], a);
    assert_eq!(&mut [4, 5, 6], b);
}

fn _unsafe4() {
    let address = 0x01234usize;
    let r = address as *mut i32;
    let values = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

extern "C" {
    fn add(input: i32) -> i32;
}

fn _extern() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", add(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function form C!");
}

static HELLO_WORLD: &str = "Hello, World!";

fn _static() {
    println!("name is {}", HELLO_WORLD)
}

static mut COUNTER: i32 = 0;

fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

fn _static2() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn _operator_overloading() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    )
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously");
    }
}

fn _trait() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn _trait2() {
    println!("A baby dog is called: {}", <Dog as Animal>::baby_name())
}

trait OutlinePrint: Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

fn _trait3() {
    let point = Point { x: 1, y: 3 };
    point.outline_print()
}

struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn _trait4() {
    let w = Wrapper(vec![String::from("Hello"), String::from("word")]);
    println!("w= {}", w);
}

type Kilometers = i32;

fn _alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}

type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}

fn returns_long_type() -> Thunk {
    Box::new(|| ())
}

fn _alias2() {
    let f: Thunk = Box::new(|| println!("hi"));
}

fn generic<T>(t: T) {}

fn generic2<T: Sized>(t: T) {}

fn generic3<T: ?Sized>(t: &T) {}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn _fn() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer)
}

fn _fn2() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>();
}

enum Status {
    Value(u32),
    Stop,
}

fn _fn3() {
    let list_of_status = (0u32..20).map(Status::Value).collect::<Vec<Status>>();
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn _fn4() {
    let closure = return_closure();
    let value = closure(1) as i32;
    println!("{}", value);
}

fn _macro() {
    let vec = my_vec![1, 2, 3];
}

fn main() {
    _fn4();
}
