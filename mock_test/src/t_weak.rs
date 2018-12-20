use std::cell::RefCell;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: char,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<{} [", self.value)?;
        let children = self.children.borrow();
        for c in children.iter() {
            write!(f, "{}", c)?;
        }
        write!(f, "]>")
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop: {}", self.value);
    }
}

fn test_branch() {
    let leaf = Rc::new(Node{
        value: 'l',
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(">>");
    println!("leaf strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node{
            value: 'b',
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!(">>");
        println!("branch strong = {}, weak = {}",
                 Rc::strong_count(&branch),
                 Rc::weak_count(&branch));
        println!("leaf strong = {}, weak = {}",
                 Rc::strong_count(&leaf),
                 Rc::weak_count(&leaf));
    }

    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(">>");
    println!("leaf strong = {}, weak = {}",
             Rc::strong_count(&leaf),
             Rc::weak_count(&leaf));
    match leaf.parent.borrow().upgrade() {
        Some(n) => println!("leaf parent = {}", n),
        None => println!("leaf parent = None"),
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work0() {
        println!(">>>");
        test_branch();
    }
}
