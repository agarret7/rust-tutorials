#[derive(Debug)]
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Received: {:?}", self);
    }
}

pub fn web_ready() -> IpAddr {
    // test constructors
    let _loopback = IpAddr::V6(String::from("::1"));
    let _move_msg = Message::Move { x: 50, y: 25 };
    let _change_color_msg = Message::ChangeColor(10, 20, 30);

    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;

    let home = IpAddr::V4(127, 0, 0, 1);
    home
}

pub fn route(ip: &IpAddr, msg: Message) {
    println!("routing to ip: {:?}", ip);
    msg.call();
}

#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    _Alabama,
    _Alaska,
    Arizona,
}

pub enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
    _HalfDollar,
}

pub fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::_Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        Coin::_HalfDollar => 50,
    }
}

// almost looks like Haskell :)
fn _plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}