use std::sync::mpsc::Sender;

pub fn zeros(s: String, n: usize) -> bool {
    match s.find(|c: char| c != '0' && c.is_alphanumeric()) {
        Some(index) => index >= n,
        None => s.len() >= n
    }
}

pub fn bruteforce(first: i64, step: usize, zero: usize, tx: Sender<i64>) {
    let mut str_rep;
    let mut digest;
    for i in (first..).step_by(step) {
        str_rep = format!("{}", i);
        digest = md5::compute(str_rep);
        if zeros(format!("{:x}", digest), zero) {
            tx.send(i).unwrap();
        }
    }
}