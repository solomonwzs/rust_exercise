// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6
// }

#[derive(Debug)]
pub enum IpAddr {
    V4(String),
    V6(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
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
}
