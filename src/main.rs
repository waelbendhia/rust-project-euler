mod problem;
mod level5;
use problem::Show;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("{}", level5::problem179::problem().show());
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
