# Rust Programming language for safety and speed

![image info](rust.png)

## Introduction

Rust is a non-garbage collected, compiled systems programming language with strong typing. It began as a personal project
by a Mozilla research employee in 2006, and later, Mozilla started sponsoring the project in 2009. Eventually, Mozilla 
adopted Rust for Firefox and rewrote the core in Rust in 2017. Rust is an advanced language with numerous features that 
developers appreciate. Positioned as a compelling alternative to C/C++, Rust offers robust capabilities for system-level programming.

## Why Rust?

C and C++ have dominated systems programming for decades. If we consider the Linux kernel, the Android Open Source Project, 
and Windows internals, all of them are written in C or C++. Rust stands out as the only language currently providing a safer alternative.

Adoptions:

* In 2021, it was [announced](https://security.googleblog.com/2021/04/rust-in-android-platform.html) that Rust adoption for Android is underway,
with the statement that about 70% of Android’s high-severity security vulnerabilities are related to memory safety.
* Microsoft is also swiftly [transitioning to Rust](https://youtu.be/8T6ClX-y2AE?t=2610) for Windows core components and business services. 
Microsoft announced the plan for the transition and released the [Rust SDK for Windows](https://learn.microsoft.com/en-us/windows/dev-environment/rust/). 
Now, writing Windows applications in Rust is more accessible. Microsoft's security team has reported a 5-15% speed improvement in components ported to Rust.
* In 2023, Linus Torvalds accepted Rust support for the Linux kernel, one of the most sophisticated codebases with millions of lines written entirely in C.

## Main Features

* Safety at compile time
* Fearless concurrency
* Speed

## Variables in Rust

```rust
fn main() {
    let x = 18; // Immutable variable
}
```

```rust
fn main() {
    let mut y = 18; // Mutable variable
}
```

We define a new variable using the `let` keyword.

Variables are immutable by default, which means we cannot change the value of the variable `x` unless we explicitly make
it mutable. But why is this the case? There are several advantages to having immutable variables.

Firstly, an immutable variable can be shared across multiple threads very easily.
Additionally, it is straightforward to store them in memory without the need to perform checks.
There is no requirement to validate anything when passing around immutable variables to multiple functions.
The decision to make variables immutable is driven by considerations of speed, concurrency, and safety.
Immutable variables contribute to faster and safer code execution, especially in concurrent programming scenarios.

## Safety features in Rust

Rust will find all the following bugs during compile time, so developers do not need to worry about these issues in
production. The functions do not even need to be called.

### Usage of uninitialized variables

We cannot compile the program. Rust will detect the mutability issue during compile time because we are trying to read
the value of an uninitialized variable.

```rust
fn use_of_uninitialized_variables_are_prohibited() {
    let x: i8;
    println!("value of x is {}", x);
}
```

### Condition evaluation happens at compile time

The code below won't compile since the compiler cannot guarantee the state of the condition.

```rust
fn conditional_evaluation_err() {
    let x: i8;

    if true {
        x = 10;
    }
    println!("value of x is {}", x);
}
```

The code below will compile since the compiler can guarantee the state of the condition.

```rust
fn conditional_evaluation() {
    let x: i8;

    if true {
        x = 10;
    } else {
        x = 20;
    }
    println!("value of x is {}", x);
}
```

### Cannot assign twice to immutable variable

```rust
fn can_not_assign_twice() {
    let x: i8 = 10;
    println!("value of x is {}", x);
    x = 20; // This line will fail; the compiler will detect the issue during compile time.
}
```

### Variables need to be defined as mutable to change the value after declaration

```rust
fn mutable_variables() {
    let mut x: i8 = 10;
    println!("value of x is {}", x);
    x = 20; // value changed here
    println!("new value of x is {}", x); // this will print the new value
}
```

### Scope in Rust

```rust
fn scope() {
    let x: i8 = 10;
    {
        let y: i8 = 20;
    }
    println!("x = {}, y = {}", x, y);
    // this will fail to compile since
    // `y` is available in a different scope in the same function
}
```

### Variable scopes and variable shadowing

```rust
fn scope_redeclaration() {
    let x: i8 = 10;

    { // New block/scope starts here
        let x: i8 = 20; // This is called shadowing
    } // The above x will be dropped here

    println!("value of x = {}", x);

    // The value of x will be 10 because the redeclaration of x is allowed in a different scope,
    // but will be dropped after the scope
}
```

## Rust Ownership

### Only 3 rules to master

* Each value has an owner. That means you cannot find any value in the memory without an owner.
* Only one owner. Only one owner for all the variable and data in the memory. It cannot be shared! But can be borrowed.
* The value gets dropped if the owner goes out of scope. There is no garbage collector to clear memory.

### Accessing moved variables will throw an error at compile time

In this example, we are trying to access the moved value. We need to understand the concept of stack and heap memory to
understand the concept here. In Rust, the datatype consists of a capacity, length, and pointer to the actual memory in
the heap. So whenever we assign the ownership to a new variable as below, Rust actually creates a new item in the stack
by copying the length, capacity, and moving the data pointer to the new variable. At this moment, the old variable (s1)
will get dropped.

```rust
fn main() {
    let s1 = "hello world!".to_string();
    let s2 = s1; // owner changed to new_msg, and msg variable is dropped here
    // we cannot access the msg variable anymore
    println!("{}", s1); // this line will throw an error "value borrowed here after move"
}
```

### Mutable move operation

Basics:

* A variable consists of two parts: stack and heap.
* Stack memory is really fast, but the size must be constant. It is basically a FIFO (First in, first out) data
  structure.
* Heap memory is slow. But can hold a large amount of data, for example, strings.

This code will work since the variable s1 is mutable. Mutable variables can be reassigned with new values and reused.

```rust
fn mutable_move_operation() {
    let mut s1 = "hello world!".to_string();
    let s2 = s1; // owner changed to s2
    // But we can assign a new value to the s1 since it is mutable
    s1 = "something else".to_string();
    println!("{}", s1); // This will work
}
```

### Cloning variables

Then how to make copies of variables? It is simple, clone the variable.
Cloning means we create a new variable with capacity, length, and data pointer.
But the difference here is that while cloning, data will be copied to a new memory in the heap. Then the variable will
be a pointer to the new memory location. In Rust, we always use the clone method; it is very common to do the cheap
clone
operations.

```rust
fn clone_data() {
    let s1 = "Hello world".to_string();
    let s2 = s1.clone(); // cloning operation
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
```

### Copy and Clone

* In Rust, Copy means the data in the stack will be copied, and the pointer to the heap will be updated.
* In the clone operation, stack values will be copied, and heap data will be copied to a new location, and the pointer
  will be updated to the new location.

### Dropping values

In Rust, there is no garbage collector; then how will it keep track of memory? It is fairly simple and straightforward.
When a variable goes out of scope, immediately the destructor method of that item will be called, then free the heap
memory,
and the stack memory will be popped. That's all, No memory leaks! no need to garbage collect the unused memory.
In other GC languages the GC must be run at specific intervals to do this; for instance, in Python,
it uses a method called reference counting to keep track of the memory allocation and

cleanup.

### Moving variable to another function

```rust
fn moving_to_functions() {
    let s1 = "message 1".to_string();
    another_function(s1); // the variable is moved here and not returning anything
    println!("{}", s1);  // this will fail to compile since the ownership moved and variable dropped
}

fn another_function(msg: String) {
    println!("msg is {}", msg)
}
```

One workaround for the above problem is to make `s1` mutable and return something from the function call and resign
the `s1` with the function return.
But there is a better way to fix this.

## Reference and Borrowing

Instead of moving the value and ownership of a variable, we can pass a reference to the value. That means, we are not
actually moving the value to the new function. So, once the new reference goes out of scope, the reference will
go out of scope, not the value or ownership. Under the hood whenever we create a new reference Rust creates a pointer
in the background and points it to the actual variable.

There are two type of references;

* Immutable reference
* Mutable reference

Rules;

* At any given time there should be only one mutable reference to a variable.
* But we can create infinite number of immutable references.
* The references can not point to null, Rust will take care of the creation and destruction of the references. This
  is achieved by a concept called lifetimes.

The reference is managed internally by using pointers like in C. But in Rust we do not need to deal the pointers
directly
The pointer de-referencing happens in the background. So we can use the `.` operator and call the methods on the
reference
object. But If needed we can also use the `*` in front of the reference to get the actual object and change the
value(only for mutable reference) etc

### Immutable reference

Immutable references are created by passing the reference as you can see in the example below. We use `&` to pass
reference
and use `&String` in the function argument. You can also see how the de-referencing happens and calling the `is_empty`
method is called
on the reference.

```rust
fn main() {
    println!("Reference and Borrowing in Rust");
    let currency = "Euro".to_string();
    println!("1. Default currency is {currency}");
    check_currency(&currency); // Passing only the reference to another function
    println!("2. Default currency is {currency}"); // It will works without any issue
}

// Accept a immutable reference
fn check_currency(currency: &String) {
    println!("Checking {currency}");
    if currency.is_empty() {
        println!("Invalid")
    }
}
```

### Mutable reference

Immutable references can be created by using a special syntax `&mut variable`. In the example below we are passing a mutable reference
to an `update_currency`. In the `update_currency` function we are de-referencing the pointer and assigning a new value. 

```rust

// Mutable reference example
fn mutable_reference() {
    let mut currency = "".to_string();
    println!("1. Default currency is {currency}");
    update_currency(&mut currency); // Passing the mutable reference to another function
    println!("2. Default currency is {currency}"); // Print the updated value
}

fn update_currency(currency: &mut String) {
  println!("Checking {currency}");
  if currency.is_empty() {
    println!("Currency is {currency}");
    *currency = "SEK".to_string();
  }
}
```
The Borrow Checking mechanism is highly beneficial when working with concurrent applications. It allows us to safely pass
references and eliminates the need to worry about safety issues. By adhering to borrowing rules, we can confidently pass 
variables between threads. However, if these rules are not followed, the code will not compile. This approach aids in identifying
potential memory leaks during the compilation process.