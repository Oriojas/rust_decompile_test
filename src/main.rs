use ethabi::Contract;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use serde_json::Value;
use std::fs;
use std::io;

async fn terminal_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ethers::middleware::Middleware;
    let sep = "==============================";

    let rpc_url = "https://carrot.megaeth.com/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    println!("{}", sep);
    println!("Enter the contract address (hex string with or without 0x prefix):");
    let address_string = terminal_input().await;
    let contract_address: Address = address_string.parse()?;
    println!("Fetching contract code...");

    let code = provider.get_code(contract_address, None).await?;

    if code.is_empty() {
        println!("No contract code found at address: {:?}", contract_address);
    } else {
        println!("Contract code found (length: {} bytes)", code.len());
    }

    println!("{}", sep);
    println!("Enter the call data (hex string with or without 0x prefix):");
    let call_data = terminal_input().await;
    println!("Call data: {}", call_data);

    println!("{}", sep);

    let abi_path =
        "/home/oscar/Github/rust_decompile_test/src/88db4f994915a64516f296fe7b9cdfa9.json";
    let abi_string = fs::read_to_string(abi_path)?;

    let json: Value = serde_json::from_str(&abi_string)?;

    let abi_json = &json["output"]["contracts"]["contracts/1_Storage.sol"]["Storage"]["abi"];

    let contract = Contract::load(abi_json.to_string().as_bytes())?;

    let call_data_bytes = hex::decode(call_data.strip_prefix("0x").unwrap_or(&call_data))?;

    if let Some(functions) = contract.functions.get("store") {
        let function = &functions[0];
        let result = function.decode_input(&call_data_bytes[4..])?;
        println!("Calling function store with arguments: {:?}", result);
    } else {
        println!("Function \'store\' not found in ABI");
    }

    Ok(())
}
