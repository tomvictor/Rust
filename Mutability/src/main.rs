

//
// We can not compile the program. Rust will detect the mutability issue in the runtime.
// Because we are trying to read value of and uninitialized variable
// fn use_of_uninitialized_variables_are_prohibited(){
//     let x:i8;
//     println!("value of x is {}", x)
// }


// cannot assign twice to immutable variable
// fn can_not_assign_twice(){
//     let x:i8 = 10;
//     println!("value of x is {}", x);
//     x = 20; // This line will fail, compiler will detect the issue in the compile time.
// }


// Scopes in rust
// fn scope_example(){
//     let x:i8 = 10;
//     {
//         let y:i8 = 20;
//     }
//     println!("x = {}, y = {}",x,y); // `y` is available in a different scope in the same function
// }

fn scope_redeclaration(){
    let x:i8 = 10;

    {
        let x:i8 = 20;
    }
    println!("value of x = {}", x);
    // The value of x will be 10, because the redeclaration of x is allowed in different scope,
    // But will be dropped after the scope
}

fn main() {
    println!("Rust Safety features!");
    scope_redeclaration();
}

