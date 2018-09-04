use std::collections::HashMap;

pub fn test_hashmap() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);

    // let mut scores = HashMap::new();
    // let s = String::from("aa");
    // scores.insert(String::from("Blue"), s);
    // let s = String::from("aa");
    // scores.entry(String::from("Blue")).or_insert(s);
    // println!("{}", s);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_hashmap();
    }
}
