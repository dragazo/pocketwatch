use std::time::Instant;
use std::io::Write;

use clap::Parser;
use time::OffsetDateTime;

#[derive(Parser, Debug)]
enum Mode {
    /// Read the current time.
    Read,
    /// Start a timer that counts up (unbounded).
    CountUp,
}

fn main() {
    match Mode::parse() {
        Mode::Read => {
            println!("{}", OffsetDateTime::now_local().unwrap());
        }
        Mode::CountUp => {
            let start = Instant::now();
            let sleep_time = std::time::Duration::from_millis(500);
            loop {
                let mut t = start.elapsed().as_secs();
                let (s, m);
                (s, t) = (t % 60, t / 60);
                (m, t) = (t % 60, t / 60);
                let h = t;

                print!("\r{h:>02}:{m:>02}:{s:>02}");
                std::io::stdout().lock().flush().unwrap();
                std::thread::sleep(sleep_time);
            }
        }
    }
}
