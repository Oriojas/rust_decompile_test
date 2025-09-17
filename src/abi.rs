use ethabi::Contract;
use ethers::types::Address;
use log::{error, info};
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;

pub async fn fetch_abi_from_arbiscan(
    contract_address: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    info!(
        "🌐 Solicitando ABI de Arbiscan para contrato: {}",
        contract_address
    );
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

    let client = Client::new();
    info!("📤 Enviando solicitud a Arbiscan API");
    let response = client.get(&url).send().await?;
    let json: Value = response.json().await?;

    if json["status"] == "1" {
        info!("✅ ABI obtenido exitosamente de Arbiscan");
        let abi_string = json["result"].as_str().unwrap();
        let abi: Value = serde_json::from_str(abi_string)?;
        Ok(abi)
    } else {
        let error_msg = json["message"].as_str().unwrap_or("Error desconocido");
        error!("❌ Error al obtener ABI de Arbiscan: {}", error_msg);
        Err(format!("Error al obtener ABI: {}", error_msg).into())
    }
}

pub async fn get_or_fetch_abi(
    contract_address: &Address,
) -> Result<(Contract, Value), Box<dyn std::error::Error>> {
    let abi_dir = "ABI";
    let abi_filename = format!("{}.json", contract_address);
    let abi_path = Path::new(abi_dir).join(&abi_filename);

    if !Path::new(abi_dir).exists() {
        info!("📁 Creando directorio ABI: {}", abi_dir);
        fs::create_dir_all(abi_dir)?;
    }

    if abi_path.exists() {
        info!("📖 Cargando ABI desde archivo local: {}", abi_filename);
        let abi_string = fs::read_to_string(&abi_path)?;
        let abi: Value = serde_json::from_str(&abi_string)?;
        let contract = Contract::load(abi.to_string().as_bytes())?;
        info!("✅ ABI cargado exitosamente desde archivo local");
        return Ok((contract, abi));
    }

    let address_string = format!("{:?}", contract_address);
    info!(
        "🌐 ABI no encontrado localmente, buscando en Arbiscan: {}",
        address_string
    );

    match fetch_abi_from_arbiscan(&address_string).await {
        Ok(abi) => {
            info!("✅ ABI obtenido exitosamente de Arbiscan");
            let abi_string = serde_json::to_string_pretty(&abi)?;
            fs::write(&abi_path, &abi_string)?;
            info!("💾 ABI guardado en archivo local: {}", abi_filename);
            let contract = Contract::load(abi_string.as_bytes())?;
            Ok((contract, abi))
        }
        Err(e) => {
            error!("❌ Error al obtener ABI de Arbiscan: {}", e);
            Err(format!("No se pudo obtener el ABI: {}. Asegúrate de que el contrato esté verificado en Arbiscan Sepolia.", e).into())
        }
    }
}
