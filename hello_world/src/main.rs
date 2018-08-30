struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
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
}

fn main() {
    test_first_word();
    test_struct();
}
