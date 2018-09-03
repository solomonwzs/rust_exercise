#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    pub fn area(&self) -> u32 {
        self.height * self.width
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
}
