# Rust learning materials

To run code;

```shell
cargo run -p variables
```

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

# Rust Ownership

## Only 3 rules to master

* Each value has an owner. That means you can not find any value in the memory without an owner.
* Only one owner. Only one owner for all the variable and data in the memory. it can not be shared! But can be borrowed.
* The value gets dropped if the owner goes out of scope. There is no garbage collector to clear memory.

## Accessing moved variables will throw error at compiletime

In this example we are trying to access the moved value. We need to understand the concept of stack and heap memory inorder
to understand the concept here. In Rust datatype consist of a capacity, length and pointer to the actual memory in the heap.
So whenever we assign the ownership to a new variable as below, rust actually create a new item in the stack by copying
the length, capacity and moving the data pointer to the new variable. At this moment the old variable(s1) will get dropped.


```rust
fn main() {
    let s1 = "hello world!".to_string();
    let s2 = s1; // owner changed to new_msg and msg variable is dropped here
    // we can not access the msg variable anymore
    println!("{}",s1); // this line will throw error "value borrowed here after move"
}
```

## Mutable move operation

Basics:
* variable consist of two parts stack and heap.
* Stack memory is really fast, but the size must be constant. it is basically a FIFO(First in first out) data structure.
* Heap memory is slow. But can hold large amount of data, for example strings.

This code will work since the variable s1 mutable. mutable variables can be reassigned with new values and reuse.
```rust
fn mutable_move_operation(){
    let mut s1 = "hello world!".to_string();
    let s2 = s1; // owner changed to s2
    // But we can assign a new value to the s1, since it is mutable
    s1 = "something else".to_string();
    println!("{}",s1); // This will work
}
```

## Cloning variables

Then how to make copies of variables? It is simple, clone the variable. Cloning means we create a new variable with
capacity, length and data pointer. but the difference here is that while cloning, data will be copied to a new memory in 
heap. Then the variable will be pointer to the new memory location. In rust, we always use the clone method, it is very
common to do the cheap clone operations.

```rust
fn clone_data(){
    let s1 = "Hello world".to_string();
    let s2 = s1.clone(); // cloning operation
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
```

## Copy and Clone

* In rust Copy means the data in the stack will be copied and the pointer to the heap will be updated.
* In clone operation, stack values will be copied and heap data will be copied to new location and the pointer will be
updated to the new location.

## Dropping values

In Rust there is no garbage collector, then how will it keep track of memory. It is fairly simple and straight forward.
When a variable goes out of scope, immediately the destructor method of that item will be called then free the heap memory,
and the stack memory will be popped. That's all, No memory leaks! no need to garbage collect the unused memory. In other GC languages the 
GC must be run in specific intervals to do this, for instance in python it uses a method called reference counting for keep track of
the memory allocation and cleanup.

## Moving variable to another function

```rust
fn moving_to_functions(){
    let s1 = "message 1".to_string();
    another_function(s1); // the variable is moved here and not returning anything
    println!("{}", s1);  // this will fail to compile, since the ownership moved and variable dropped
}

fn another_function(msg: String){
    println!("msg is {}", msg)
}
```

One work around for the above problem is to make s1 mutable, and return something from the function call and
resign the s1 with the function return. But there is better way to fix this.