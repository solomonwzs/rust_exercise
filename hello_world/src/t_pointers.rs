use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::ops::Deref;

enum List {
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

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        println!("deref: {}", self.0);
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop: {}", self.0);
    }
}

#[cfg(test)]
mod test {
    use super::List::{Cons, Nil};
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{}", list);
    }

    #[test]
    fn it_work0() {
        println!(">>>");
        let x = 5;
        let y = MyBox::new(x);
        // let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);

        println!("y: {}", *y);
        drop(y);
        println!("===");
    }
}
