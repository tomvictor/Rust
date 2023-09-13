
fn main() {
    // Variables and Mutability
    let mut y = 5;
    println!("The value of x is: {y}");
    y = 6;
    println!("The value of x is: {y}");

    // Constants
    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;

    println!("THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");


    // Shadowing : can use the same variable and scope

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // change datatype at runtime
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces : {spaces}");

    // Floating point

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;


    // The Boolean Type
    let t:bool = true;
    let f: bool = false; // with explicit type annotation

    // The Character Type
    let c: char = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{heart_eyed_cat}");

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructure a tuple value
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // We can also access a tuple element directly by using a period (.)
    // followed by the index of the value we want to access.
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // The Array Type
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // The array named a will contain 5 elements that will all be set to the value 3 initially.
    // This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
    let a = [3; 5];
    // Accessing Array Elements
    let first = a[0];
    let second = a[1];

    // functions
    say_something();
    let value:i32 = get_value();
    println!("{value}");
    another_function(5);
}

fn say_something(){
    println!("hello.....");
}

fn get_value()->i32{
    let mut c:i32 = 100;
    // no semicolon in the return
    c+100
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
