use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use ethabi::{Contract, Token};
use ethers::types::Address;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;

// Struct para la petición JSON entrante
#[derive(Deserialize)]
struct DecodeRequest {
    contract_address: String,
    call_data: String,
}

// Struct para la respuesta JSON saliente
#[derive(Serialize)]
struct DecodeResponse {
    status: String, // "success" or "error"
    function_name: Option<String>,
    arguments: Option<Vec<String>>, // Represent arguments as strings for simplicity
    message: Option<String>,
    details: Option<String>, // For additional error info
}

async fn fetch_abi_from_arbiscan(
    contract_address: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
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

async fn get_or_fetch_abi(
    contract_address: &Address,
) -> Result<Contract, Box<dyn std::error::Error>> {
    let abi_dir = "ABI";
    let abi_filename = format!("{}.json", contract_address);
    let abi_path = Path::new(abi_dir).join(&abi_filename);

    if !Path::new(abi_dir).exists() {
        fs::create_dir_all(abi_dir)?;
    }

    if abi_path.exists() {
        let abi_string = fs::read_to_string(&abi_path)?;
        let abi: Value = serde_json::from_str(&abi_string)?;
        let contract = Contract::load(abi.to_string().as_bytes())?;
        return Ok(contract);
    }

    let address_string = format!("{:?}", contract_address);

    match fetch_abi_from_arbiscan(&address_string).await {
        Ok(abi) => {
            let abi_string = serde_json::to_string_pretty(&abi)?;
            fs::write(&abi_path, &abi_string)?;
            let contract = Contract::load(abi_string.as_bytes())?;
            Ok(contract)
        }
        Err(e) => {
            Err(format!("No se pudo obtener el ABI: {}. Asegúrate de que el contrato esté verificado en Arbiscan Sepolia.", e).into())
        }
    }
}

fn decode_function_call(
    contract: &Contract,
    call_data: &str,
) -> Result<(String, Vec<Token>), Box<dyn std::error::Error>> {
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
                        return Ok((name.clone(), result));
                    }
                    Err(_) => {}
                }
            }
        }
    }

    Err(format!(
        "No se encontró función coincidente para el selector: 0x{}",
        hex::encode(function_selector)
    )
    .into())
}

async fn decode_handler(req: web::Json<DecodeRequest>) -> impl Responder {
    let contract_address_result = req.contract_address.parse::<Address>();
    let contract_address = match contract_address_result {
        Ok(addr) => addr,
        Err(e) => {
            return HttpResponse::BadRequest().json(DecodeResponse {
                status: "error".to_string(),
                function_name: None,
                arguments: None,
                message: Some(format!("Dirección de contrato inválida: {}", e)),
                details: None,
            });
        }
    };

    let contract = match get_or_fetch_abi(&contract_address).await {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::InternalServerError().json(DecodeResponse {
                status: "error".to_string(),
                function_name: None,
                arguments: None,
                message: Some("Error al obtener o cargar el ABI".to_string()),
                details: Some(e.to_string()),
            });
        }
    };

    match decode_function_call(&contract, &req.call_data) {
        Ok((name, args)) => {
            let args_str: Vec<String> = args.into_iter().map(|arg| format!("{:?}", arg)).collect();
            HttpResponse::Ok().json(DecodeResponse {
                status: "success".to_string(),
                function_name: Some(name),
                arguments: Some(args_str),
                message: None,
                details: None,
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(DecodeResponse {
            status: "error".to_string(),
            function_name: None,
            arguments: None,
            message: Some("Error al decodificar los datos de llamada".to_string()),
            details: Some(e.to_string()),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let server_address = "127.0.0.1:8080";

    println!("🚀 Servidor web iniciando en http://{}", server_address);

    HttpServer::new(|| App::new().route("/decode", web::post().to(decode_handler)))
        .bind(server_address)?
        .run()
        .await
}
