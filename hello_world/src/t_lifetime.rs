pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn longest0(x: &mut String, y: &mut String) {
    if x.len() > y.len() {
        println!("{}", x);
    } else {
        println!("{}", y);
    }
}

pub fn test_lifetime() {
    let mut x = String::from("中文中文");
    let mut y = String::from("qwert");
    longest0(&mut x, &mut y);

    let x = "1234";
    let y = "09876";
    let z = longest(x, y);
    println!("{}", z);
    println!("{}", x);
    println!("{}", y);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_lifetime();
    }
}
