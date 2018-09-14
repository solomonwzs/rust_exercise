use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::Rc;

enum List1 {
    Cons(i32, Rc<List1>),
    Nil,
}

enum List2<'a> {
    Cons(i32, &'a List2<'a>),
    Nil,
}

impl Display for List1 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            List1::Cons(i, next) => write!(f, "{}, {}", i, next),
            List1::Nil => write!(f, "nil"),
        }
    }
}

impl<'a> Display for List2<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            List2::Cons(i, next) => write!(f, "{}, {}", i, next),
            List2::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        use super::List1::{Cons, Nil};

        println!(">>>");
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));

        println!("{}", Rc::strong_count(&a));

        println!("a: {}", a);
        println!("b: {}", b);
        println!("c: {}", c);
    }

    #[test]
    fn it_work0() {
        use super::List2::{Cons, Nil};

        println!(">>>");

        let a = Cons(5, &Cons(10, &Cons(1, &Nil)));
        let b = Cons(3, &a);
        let c = Cons(4, &a);

        println!("a: {}", a);
        println!("b: {}", b);
        println!("c: {}", c);
    }
}
