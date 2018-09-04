pub fn test_vectors() {
    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(1);
    println!("{:#?}", does_not_exist);

    let mut s = String::from("hello");
    {
        let mut v = Vec::new();
        v.push(&mut s);
        println!("{:#?}", v);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_vectors();
    }
}
