struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn _user() {
    let mut user1 = build_user(
        String::from("username"),
        String::from("someone@example.com"),
    );

    user1.email = String::from("another_one@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("User 2: {}", user2.email);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn _tuple() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black: {}, {}, {}", black.0, black.1, black.2);
}

struct AlwaysEqual;

fn _empty() {
    let subject = AlwaysEqual;
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn _rectangle() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn _rect() {
    let rect = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn _rectangle2() {
    let scale = 2;
    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rectangle)
    );
    println!("rectangle is {:#?}", rectangle);
    dbg!(&rectangle);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn _rectangle3() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );
    dbg!(&rectangle);
}

fn _rectangle4() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn _square() {
    let square = Rectangle::square(3);
    println!("square is {:#?}", square);
    println!("area of square is {:#?}", square.area());
}

fn main() {
    _square();
}
