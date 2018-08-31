#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    // fn can_hold(&self, other: &Rectangle) -> bool {
    //     self.width > other.width && self.height > other.height
    // }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test_first_word() {
    let s = String::from("hello world");
    let word = first_word(&s);

    // s.clear();

    println!("[{}]", s);
    println!("[{}]", word);

    // let s = &s[20..];
    // println!("[{}]", s);
}

fn test_struct() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("{:#?}", user1);

    let mut rect1 = Rectangle { width: 30, height: 50 };
    println!("{}", area(&rect1));

    let tmp = &mut rect1;
    println!("{}", tmp.area());
}

fn test_enum() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let x: Option<i32> = None;
    let y: Option<i32> = Some(1);
    println!("{}", x == y);

    match y {
        Some(i) => println!("{}", i),
        None => println!("None"),
    }

    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

fn main() {
    test_first_word();
    test_struct();
    test_enum();
}
