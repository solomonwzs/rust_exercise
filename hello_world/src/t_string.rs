pub fn test_string() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is [{}]", s2);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("[{}]", s);

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_string();
    }
}
