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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_first_word();
    }
}
