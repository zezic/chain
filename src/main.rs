use std::{process::Command, time::Duration};

fn main() {
    let seq = std::env::var("SEQ").expect("no variable");
    let mut chars = seq.chars();
    let maybe_digit = chars.next();

    if let Some(digit) = maybe_digit {
        let next_seq: String = chars.collect();
        std::env::set_var("SEQ", next_seq);
        let mut child = Command::new(digit.to_string())
            .spawn()
            .expect("spawning next command");
        let _status = child.wait().expect("waiting");
    } else {
        // Infinite sleep for last chain segment
        loop {
            std::thread::sleep(Duration::from_secs(60 * 60));
        }
    }

    println!("end");
}
