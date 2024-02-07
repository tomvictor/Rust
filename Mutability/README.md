# Rust Mutability

Rust will find all the following bugs in during the compile time, so that the developer do not need to worry about these
issues in the production. The functions do not even need to be called.

## Usage of  uninitialized variables

We can not compile the program. Rust will detect the mutability issue in the compiletime itself.
Because we are trying to read value of and uninitialized variable.

```rust
fn use_of_uninitialized_variables_are_prohibited(){
    let x:i8;
    println!("value of x is {}", x)
}
```

## Condition evaluation happens at compile time

Code below won't compile since the compiler can not guarantee the state of condition.

```rust
fn conditional_evaluation_err(){
    let x:i8;

    if true{
        x = 10;
    }
    println!("value of x is {}", x)
}
```

Code below will compile since the compiler can guarantee the state of the condition

```rust
fn conditional_evaluation(){
    let x:i8;

    if true{
        x = 10;
    } else {
        x = 20;
    }
    println!("value of x is {}", x)
}
```

## Cannot assign twice to immutable variable

```rust
fn can_not_assign_twice(){
    let x:i8 = 10;
    println!("value of x is {}", x);
    x = 20; // This line will fail, compiler will detect the issue in the compile time.
}
```

## variables need to be defined as mutable inorder to change the value after declaration

```rust
fn mutable_variables(){
    let mut x:i8 = 10;
    println!("value of x is {}", x);
    x = 20; // value changed here
    println!("new value of x is {}", x); // this will print the new value
}
```

## Scope in Rust

```rust
fn scope(){
    let x:i8 = 10;
    {
        let y:i8 = 20;
    }
    println!("x = {}, y = {}",x,y); 
    // this will fail to compile since
    // `y` is available in a different scope in the same function
}
```

## Variable scopes and variable shadowing

```rust
fn scope_redeclaration(){
    let x:i8 = 10;
    
    { // New block/scope starts here
        let x:i8 = 20; // This is called shadowing,
    } // The above x will be dropped here
    
    println!("value of x = {}", x);
    
    // The value of x will be 10, because the redeclaration of x is allowed in different scope,
    // But will be dropped after the scope
}
```