#[derive(Debug)]
pub enum BetOption {
    Option1,
    Option2,
}

fn main() {
    println!("Enums");

    const input: u8 = 1;

    let bet_option: BetOption = {
        match input {
            0 => BetOption::Option1,
            1 => BetOption::Option2,
            _ => panic!("invalid input"),
        }
    };
    println!("selected option: {:?}", bet_option);

    let amount: f64 = 1.0;
    // Transfer amount, adjust for decimals
    let result = amount * 10u64.pow(9) as f64;
    println!("result : {:?}", result);
}
