use std::thread;
// use std::time::Duration;

pub fn run(){
    println!("This is a demo function");
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            let resp = reqwest::blocking::get("https://httpbin.org/ip");
            println!("{:#?}", resp);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join().unwrap();
}
