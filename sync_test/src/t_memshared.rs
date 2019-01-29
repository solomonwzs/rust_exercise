use std::sync::{Arc, Mutex};
use std::thread;

fn t_mem_shared() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn t_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_work() {
        println!(">>>");
        t_mem_shared();
    }

    #[test]
    fn it_work0() {
        println!(">>>");
        t_move();
    }
}
