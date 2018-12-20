use std::cell::RefCell;
use std::rc::Rc;
use self::List1::{Cons, Nil};

#[derive(Debug)]
enum List1 {
    Cons(char, RefCell<Rc<List1>>),
    Nil,
}

impl Drop for List1 {
    fn drop(&mut self) {
        match self {
            Cons(ch, _) => println!("xxx: {}", ch),
            Nil => println!("xxx: nil"),
        }
    }
}

impl List1 {
    fn tail(&self) -> Option<&RefCell<Rc<List1>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn test_ref_cycles() {
    let a = Rc::new(Cons('a', RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons('b', RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // if let Some(link) = a.tail() {
    //     *link.borrow_mut() = Rc::clone(&b);
    // }

    println!("> b rc count after changing a = {}", Rc::strong_count(&b));
    println!("> a rc count after changing a = {}", Rc::strong_count(&a));

    // panic!
    // println!("a next item = {:?}", a.tail());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        test_ref_cycles();
        println!(">>>");
    }
}
