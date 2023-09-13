use std::boxed::Box;

fn main() {
    println!("Rust Pointers!");
    
    let a = [0; 10];
    println!("a: {:?}", a);
    let b = a;
    println!("b: {:?}", b);
    // println("a: {:?}",a); // Error, already moved to b

    let readings:i32 = 9;
    let c = Box::new([readings;10]);
    println!("box c: {:?}", c);

    let result = modify(c);
    println!("result: {:?}", result);

    // println!("old c: {:?}", c); // this will not work

}

fn modify(values: Box<[i32]>) -> Box<[i32]> {
    let new_reading:i32 = 369;
    let modified_box = Box::new([new_reading;10]);
    modified_box
}