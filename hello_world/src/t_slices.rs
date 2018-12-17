pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


pub fn test_first_word() {
    let s = String::from("hello world");
    let word = first_word(&s);

    // s.clear();

    println!("[{}]", s);
    println!("[{}]", word);

    // let s = &s[20..];
    // println!("[{}]", s);
}


pub fn test_slices() {
    let mut s0 = String::from("hello");
    s0.push_str("world");
    let s1 = &s0[0..3];
    // s0.push_str("!");

    println!("[{}], [{}]", s0, s1);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_first_word();
        test_slices();
    }
}
