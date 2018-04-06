mod problem;
mod level7;
mod level9;
use std::io;
use problem::Show;
use std::time::{Duration, Instant};

fn main() {
    let mut all_solutions = level7::solutions();
    all_solutions.append(&mut level9::solutions());
    loop {
        println!("Select a problem or 0 to exit.");
        let sel = read_num();
        if sel == 0 {
            return;
        } else {
            match all_solutions.iter().find(|&s| s.ind == sel) {
                Some(s) => {
                    let start = Instant::now();
                    println!("{}", s.show().to_string());
                    let duration = start.elapsed();
                    println!("Time elapsed is: {} s", fractional_duration(duration));
                }
                None => println!("Problem {} not solved.", sel),
            }
        }
    }
}

fn read_num() -> u32 {
    let mut buffer = String::new();
    loop {
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => match buffer.to_string().trim().parse::<u32>() {
                Ok(n) => return n,
                Err(_) => println!("Please write a number"),
            },
            Err(_) => println!("I couldn't understand you"),
        }
    }
}

fn fractional_duration(dur: Duration) -> f64 {
    let secs = dur.as_secs();
    let nanos = dur.subsec_nanos();
    return (secs as f64) + (nanos as f64 / 10u32.pow(9) as f64);
}
