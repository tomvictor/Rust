/// T and U represent the generic types. It can be any type but must implement the
/// Display trait
fn log_data<T: std::fmt::Display, U: std::fmt::Display>(key: T, value: U) {
    println!("{:}: {:}", key, value);
}

/// This function is also same as the above function but the only
/// difference is in the where clause. it is also possible to use the
/// where clause instead of the inline notation. Using the where clause we can
/// chain multiple trait implementations and it is a bit more clean
fn log_data_from<T, U>(key: T, value: U)
where
    T: std::fmt::Display + std::fmt::Debug,
    U: std::fmt::Display + std::fmt::Debug,
{
    println!("{:?}: {:?}", key, value);
}

fn main() {
    println!("Generics basics");
    log_data("hello", "world");
    log_data_from("hello", "generics")
}
