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

impl IpAddr {
    pub fn foo(&self) -> &String {
        match self {
            IpAddr::V4(s) => s,
            IpAddr::V6(s) => s,
        }
    }
}

pub fn test_enum() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:#?}", home);
    println!("{:#?}", loopback);

    let x: Option<i32> = None;
    let y: Option<i32> = Some(1);
    println!("{:#?}", x);
    println!("{:#?}", y);
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_enum();
    }
}
