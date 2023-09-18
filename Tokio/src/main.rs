use std::{thread};
use std::time::Duration;
async fn process(){
    print!("processing!")
}

fn main() {
    println!("Starting..");
    thread::sleep(Duration::from_secs(5));
}
