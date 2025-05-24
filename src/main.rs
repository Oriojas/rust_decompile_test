use ethabi::Contract;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use reqwest;
use serde_json::Value;
use std::env;
use std::fs;
use std::io;
use std::path::Path;

async fn terminal_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

async fn fetch_abi_from_arbiscan(contract_address: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let api_key = env::var("ARBISCAN_API_KEY").unwrap_or_default();
    
    let url = if api_key.is_empty() {
        format!(
            "https://api-sepolia.arbiscan.io/api?module=contract&action=getabi&address={}",
            contract_address
        )
    } else {
        format!(
            "https://api-sepolia.arbiscan.io/api?module=contract&action=getabi&address={}&apikey={}",
            contract_address,
            api_key
        )
    };
    
    if api_key.is_empty() {
        println!("⚠️  Usando API sin autenticación (puede tener límites)");
        println!("💡 Configura ARBISCAN_API_KEY en .env para mejor rendimiento");
    } else {
        println!("🔑 Usando API key de Arbiscan");
    }
    
    println!("📡 Consultando Arbiscan Sepolia...");
    
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let json: Value = response.json().await?;
    
    if json["status"] == "1" {
        let abi_string = json["result"].as_str().unwrap();
        let abi: Value = serde_json::from_str(abi_string)?;
        Ok(abi)
    } else {
        Err(format!("Error al obtener ABI: {}", json["message"]).into())
    }
}

async fn get_or_fetch_abi(contract_address: &Address) -> Result<Value, Box<dyn std::error::Error>> {
    let abi_dir = "ABI";
    let abi_filename = format!("{}.json", contract_address);
    let abi_path = Path::new(abi_dir).join(&abi_filename);
    
    if !Path::new(abi_dir).exists() {
        fs::create_dir_all(abi_dir)?;
    }
    
    if abi_path.exists() {
        println!("📂 Cargando ABI desde caché local: {}", abi_path.display());
        let abi_string = fs::read_to_string(&abi_path)?;
        let abi: Value = serde_json::from_str(&abi_string)?;
        return Ok(abi);
    }
    
    println!("🌐 Obteniendo ABI de Arbitrum Sepolia...");
    let address_string = format!("{:?}", contract_address);
    
    match fetch_abi_from_arbiscan(&address_string).await {
        Ok(abi) => {
            let abi_string = serde_json::to_string_pretty(&abi)?;
            fs::write(&abi_path, abi_string)?;
            println!("💾 ABI guardado en: {}", abi_path.display());
            Ok(abi)
        }
        Err(e) => {
            Err(format!("No se pudo obtener el ABI: {}. Asegúrate de que el contrato esté verificado en Arbiscan Sepolia.", e).into())
        }
    }
}

async fn decode_function_call(contract: &Contract, call_data: &str) -> Result<(), Box<dyn std::error::Error>> {
    let call_data_bytes = hex::decode(call_data.strip_prefix("0x").unwrap_or(call_data))?;
    
    if call_data_bytes.len() < 4 {
        return Err("Datos de llamada muy cortos".into());
    }
    
    let function_selector = &call_data_bytes[0..4];
    let input_data = &call_data_bytes[4..];
    
    for (name, functions) in &contract.functions {
        for function in functions {
            let computed_selector = function.short_signature();
            if computed_selector == function_selector {
                match function.decode_input(input_data) {
                    Ok(result) => {
                        println!("✅ Función encontrada: {}", name);
                        println!("📝 Argumentos: {:?}", result);
                        return Ok(());
                    }
                    Err(e) => {
                        println!("❌ Error decodificando función {}: {}", name, e);
                    }
                }
            }
        }
    }
    
    println!("❓ No se encontró función coincidente para el selector: 0x{}", hex::encode(function_selector));
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ethers::middleware::Middleware;
    
    dotenv::dotenv().ok();
    
    let sep = "==============================";

    println!("{}", sep);
    println!("🔍 Decodificador de Contratos - Arbitrum Sepolia");
    println!("{}", sep);
    
    let rpc_url = "https://sepolia-rollup.arbitrum.io/rpc";
    let provider = Provider::<Http>::try_from(rpc_url)?;

    println!("📍 Ingresa la dirección del contrato (con o sin prefijo 0x):");
    let address_string = terminal_input().await;
    let contract_address: Address = address_string.parse()?;
    println!("🔍 Verificando contrato en Arbitrum Sepolia...");

    let code = provider.get_code(contract_address, None).await?;

    if code.is_empty() {
        println!("❌ No se encontró código de contrato en la dirección: {:?}", contract_address);
        return Ok(());
    } else {
        println!("✅ Código de contrato encontrado ({} bytes)", code.len());
    }

    println!("{}", sep);
    println!("📥 Obteniendo ABI del contrato...");
    
    let abi = match get_or_fetch_abi(&contract_address).await {
        Ok(abi) => abi,
        Err(e) => {
            println!("❌ Error obteniendo ABI: {}", e);
            return Ok(());
        }
    };

    let contract = Contract::load(abi.to_string().as_bytes())?;
    println!("✅ ABI cargado exitosamente!");
    
    println!("📋 Funciones disponibles en el contrato:");
    for (name, functions) in &contract.functions {
        for function in functions {
            println!("  - {}: {:?}", name, function.inputs);
        }
    }

    println!("{}", sep);
    println!("📤 Ingresa los datos de llamada (hex con o sin prefijo 0x):");
    let call_data = terminal_input().await;
    println!("📊 Datos de llamada: {}", call_data);

    println!("{}", sep);
    println!("🔓 Decodificando llamada a función...");
    
    decode_function_call(&contract, &call_data).await?;

    Ok(())
}