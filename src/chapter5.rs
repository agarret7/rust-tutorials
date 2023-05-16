#[derive(Debug)]
pub struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
pub struct Color(i32, i32, i32);

#[derive(Debug)]
pub struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub fn ready_up() -> (User, User, Color, Point) {
    let player1 = User {
        active: true,
        username: String::from("someonesname"),
        email: String::from("someonesemail@example.com"),
        sign_in_count: 1,
    };

    let player2 = User {
        active: player1.active,
        username: player1.username.clone(),
        email: String::from("another@example.com"),
        sign_in_count: player1.sign_in_count
    };

    println!("{:?}", player1.email);
    println!("{:?}", player2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:#?}", origin);
    println!("{:#?}", black);

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
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
    let rect4 = Rectangle::square(50);

    println!("rect1 is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}", rect1.can_hold(&rect4));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    return (player1, player2, black, origin);
}