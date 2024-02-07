//
// We can not compile the program. Rust will detect the mutability issue in the runtime.
// Because we are trying to read value of and uninitialized variable
// fn use_of_uninitialized_variables_are_prohibited(){
//     let x:i8;
//     println!("value of x is {}", x)
// }

// condition evaluation happens at compile time
// code below won't compile since the compiler can not guarantee the state of condition
// fn conditional_evaluation_err(){
//     let x:i8;
//
//     if true{
//         x = 10;
//     }
//     println!("value of x is {}", x)
// }

// code below will compile since the compiler can guarantee the state of condition
fn conditional_evaluation() {
    let x: i8;

    if true {
        x = 10;
    } else {
        x = 20;
    }
    println!("value of x is {}", x)
}


// cannot assign twice to immutable variable
// fn can_not_assign_twice(){
//     let x:i8 = 10;
//     println!("value of x is {}", x);
//     x = 20; // This line will fail, compiler will detect the issue in the compile time.
// }

// variables need to be defined as mutable inorder to change the value after declaration
fn mutable_variables() {
    let mut x: i8 = 10;
    println!("value of x is {}", x);
    x = 20;
    println!("new value of x is {}", x);
}


// Scopes in rust
// fn scope_example(){
//     let x:i8 = 10;
//     {
//         let y:i8 = 20;
//     }
//     println!("x = {}, y = {}",x,y); // `y` is available in a different scope in the same function
// }

fn scope_redeclaration() {
    let x: i8 = 10;

    { // New block/scope starts here
        let x: i8 = 20; // This is called shadowing,
    } // The above x will be dropped here
    println!("value of x = {}", x);
    // The value of x will be 10, because the redeclaration of x is allowed in different scope,
    // But will be dropped after the scope
}

fn main() {
    println!("Rust Safety features!");
    scope_redeclaration();
    mutable_variables();
}

