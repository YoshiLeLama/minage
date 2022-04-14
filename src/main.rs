use std::{sync::mpsc, thread};

use minage::*;

fn main() {
    let (tx, rx) = mpsc::channel::<i64>();

    for i in 0..10 {
        let tx = tx.clone();
        thread::spawn(move || {
            bruteforce(i, 10, 6, tx);
        });
    }

    for _ in 0..5 {
        let received = rx.recv().unwrap();

        println!("{}", received);
    }
}
