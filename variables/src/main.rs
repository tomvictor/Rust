
// const declaration
// type annotation is must for const
// declare in th mod level
const WARP_FACTOR:f64 = 9.9;

fn main() {
    println!("Variables!");
    // tuple
    let (bmws, vw) = (999,1000);

    // scope
    let x = 5;

    {
        let y = x;
        println!("{}, {}",x,y);
    }            // drop

    // println!("{}",y);   error - out of scope
    println!("{}", x);      // this is ok

    // memory safety at compile time

    let enigma: i32;

    // this won't work, need to initialise before use
    // println!("{}", enigma);

    // if true {
    //     enigma = 42;
    // }

    // code below also do not work, since the conditions will only be evaluated during runtime
    // println!("{}", enigma);

    if true {
        enigma = 42;
    } else {
        enigma = 30;
    }
    // this will works since the value if the variable is always guaranteed
    println!("{}", enigma);

}
