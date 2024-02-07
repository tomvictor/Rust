// Immutable reference example
fn immutable_reference() {
    let currency = "SEK".to_string();
    println!("1. Default currency is {currency}");
    check_currency(&currency); // Passing only the reference to another function
    println!("2. Default currency is {currency}"); // It will works without any issue
}


// Mutable reference example
fn mutable_reference() {
    let mut currency = "".to_string();
    println!("1. Default currency is {currency}");
    update_currency(&mut currency); // Passing the mutable reference to another function
    println!("2. Default currency is {currency}"); // Print the updated value
}


// Accept a immutable reference
fn check_currency(currency: &String) {
    println!("Checking {currency}");
    if currency.is_empty() {
        println!("Invalid")
    }
}

// Accept a mutable reference
fn update_currency(currency: &mut String) {
    println!("Checking {currency}");
    if currency.is_empty() {
        println!("Currency is {currency}");
        *currency = "SEK".to_string();
    }
}

fn main() {
    println!("Reference and Borrowing in Rust");
    // immutable_reference();
    mutable_reference();
}

