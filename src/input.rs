use std::io::{self, Write};

pub fn get_contract_address() -> String {
    print!("Please enter the contract address: ");
    io::stdout().flush().unwrap();
    let mut contract_address = String::new();
    io::stdin().read_line(&mut contract_address).expect("Failed to read line");
    contract_address.trim().to_string()
}
