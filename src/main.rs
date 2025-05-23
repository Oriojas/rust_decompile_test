use ethers::types::TransactionRequest;
use ethers::utils::rlp;
use std::io;

fn terminal_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let sep = "==============================";

    println!("{}", sep);
    println!("Enter the binary (hex string with or without 0x prefix):");
    let mut binary = terminal_input();

    if binary.starts_with("0x") {
        binary = binary[2..].to_string();
    }

    println!("{}", sep);
    println!("Decoding transaction...");

    match hex::decode(&binary) {
        Ok(tx_bytes) => match rlp::decode::<TransactionRequest>(&tx_bytes) {
            Ok(tx) => {
                println!("Transaction decoded successfully:");
                println!("To: {:?}", tx.to);
                println!("Value: {:?}", tx.value);
                println!("Gas Price: {:?}", tx.gas_price);
                println!("Gas: {:?}", tx.gas);
                println!("Input Data: {:?}", tx.data);
                println!("Nonce: {:?}", tx.nonce);
                println!("Chain ID: {:?}", tx.chain_id);
            }
            Err(e) => println!("Failed to decode RLP: {}", e),
        },
        Err(e) => println!("Invalid hex string: {}", e),
    }

    println!("{}", sep);
}
