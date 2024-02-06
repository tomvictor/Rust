


// Accessing moved values will throw error
// fn invalid_move_operation(){
//     let msg = "hello world!".to_string();
//     let new_msg = msg; // owner changed to new_msg and msg variable is dropped here
//     // we can not access the msg variable anymore
//     println!("{}",msg); // this line will throw error "value borrowed here after move"
// }

fn mutable_move_operation(){
    let mut s1 = "hello world!".to_string();
    let s2 = s1; // owner changed to s2
    // But we can assign a new value to the s1, since it is mutable
    s1 = "something else".to_string();
    println!("{}",s1); // This will work
}


// It is completely ok to clone data
fn clone_data(){
    let s1 = "Hello world".to_string();
    let s2 = s1.clone();
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}

// fn moving_to_functions(){
//     let s1 = "message 1".to_string();
//     another_function(s1); // the variable is moved here and not returning anything
//     println!("{}", s1);  // this will fail to compile, since the ownership moved and variable dropped
// }

fn another_function(msg: String){
    println!("msg is {}", msg)
}

fn main() {
    println!("Rust Ownership!");
    mutable_move_operation();
    clone_data();
}
