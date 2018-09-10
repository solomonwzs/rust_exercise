use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            List::Cons(i, next) => write!(f, "{}, {}", i, next),
            List::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::List::{Cons, Nil};

    #[test]
    fn it_work() {
        println!(">>>");
        let list = Cons(1, Box::new(Cons(2, Box::new(
                        Cons(3, Box::new(Nil))))));
        println!("{}", list);
    }
}
