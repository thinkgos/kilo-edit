use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().flatten() {
        println!("User input: {}", line);
    }
}
