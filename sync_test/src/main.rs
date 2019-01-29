use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod t_memshared;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx_copy = mpsc::Sender::clone(&tx);

    let h0 = thread::spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    let h1 = thread::spawn(move || {
        let vals = vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
            String::from("D"),
        ];

        for val in vals {
            tx_copy.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("got: {}", received);
    }
    h0.join().unwrap();
    h1.join().unwrap();
}
