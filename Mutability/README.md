# Rust Mutability

Rust will find all the following bugs in during the compile time, so that the developer do not need to worry about these 
issues in the production. The functions do not even need to be called.

## Usage of  uninitialized variables

We can not compile the program. Rust will detect the mutability issue in the compiletime itself.
Because we are trying to read value of and uninitialized variable.

    fn use_of_uninitialized_variables_are_prohibited(){
        let x:i8;
        println!("value of x is {}", x)
    }

## Cannot assign twice to immutable variable

    fn can_not_assign_twice(){
        let x:i8 = 10;
        println!("value of x is {}", x);
        x = 20; // This line will fail, compiler will detect the issue in the compile time.
    }

## Scope in Rust

    fn scope(){
        let x:i8 = 10;
            {
                let y:i8 = 20;
            }
        println!("x = {}, y = {}",x,y); // `y` is available in a different scope in the same function
    }

## Variable redeclaration and scopes

    fn scope_redeclaration(){
    let x:i8 = 10;

    {
        let x:i8 = 20;
    }
    println!("value of x = {}", x);
    // The value of x will be 10, because the redeclaration of x is allowed in different scope, 
    // But will be dropped after the scope
}