use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use ethabi::{Contract, Token};
use ethers::types::Address;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::path::Path;
use url::Url;

// Struct para la configuración del prompt
#[derive(Deserialize)]
struct PromptConfig {
    system_message: String,
    user_prompt_template: String,
    response_format: ResponseFormat,
    model_settings: ModelSettings,
}

#[derive(Deserialize)]
struct ResponseFormat {
    risk_level_prefix: String,
    explanation_prefix: String,
}

#[derive(Deserialize)]
struct ModelSettings {
    model: String,
    stream: bool,
}

// Struct para la petición JSON entrante del endpoint /decode
#[derive(Deserialize)]
struct DecodeRequest {
    contract_address: String,
    call_data: String,
}

// Struct para la respuesta JSON saliente del endpoint /decode
#[derive(Serialize)]
struct DecodeResponse {
    status: String, // "success" or "error"
    function_name: Option<String>,
    arguments: Option<Vec<String>>, // Represent arguments as strings for simplicity
    message: Option<String>,
    details: Option<String>, // For additional error info
    abi: Option<Value>,      // Include ABI in successful response for analysis endpoint
}

// Struct para la petición JSON entrante del endpoint /analysis
#[derive(Deserialize)]
struct AnalysisRequest {
    contract_address: String,
    call_data: String,
}

// Struct para la respuesta JSON saliente del endpoint /analysis
#[derive(Serialize)]
struct AnalysisResponse {
    status: String,                 // "success" or "error"
    function_name: Option<String>,  // Include decoded function name
    arguments: Option<Vec<String>>, // Include decoded arguments
    risk_level: Option<String>,     // e.g., "Low", "Medium", "High", "Caution", "Unknown"
    explanation: Option<String>,    // Explanation from the LLM
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

// Modificado para retornar Contract y ABI Value
async fn get_or_fetch_abi(
    contract_address: &Address,
) -> Result<(Contract, Value), Box<dyn std::error::Error>> {
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
        return Ok((contract, abi));
    }

    let address_string = format!("{:?}", contract_address);

    match fetch_abi_from_arbiscan(&address_string).await {
        Ok(abi) => {
            let abi_string = serde_json::to_string_pretty(&abi)?;
            fs::write(&abi_path, &abi_string)?;
            let contract = Contract::load(abi_string.as_bytes())?;
            Ok((contract, abi))
        }
        Err(e) => {
            Err(format!("No se pudo obtener el ABI: {}. Asegúrate de que el contrato esté verificado en Arbiscan Sepolia.", e).into())
        }
    }
}

// Modificado para retornar resultado en lugar de imprimir
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
                    Err(_) => {
                        // Log or handle decoding errors specifically if needed
                    }
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

// Función para cargar la configuración del prompt
fn load_prompt_config() -> Result<PromptConfig, Box<dyn std::error::Error>> {
    let config_path = "src/prompt_config.json";
    let config_content = fs::read_to_string(config_path)?;
    let config: PromptConfig = serde_json::from_str(&config_content)?;
    Ok(config)
}

// Manejador para la ruta /decode
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
                abi: None,
            });
        }
    };

    let (contract, abi) = match get_or_fetch_abi(&contract_address).await {
        Ok((c, a)) => (c, a),
        Err(e) => {
            return HttpResponse::InternalServerError().json(DecodeResponse {
                status: "error".to_string(),
                function_name: None,
                arguments: None,
                message: Some("Error al obtener o cargar el ABI".to_string()),
                details: Some(e.to_string()),
                abi: None,
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
                abi: Some(abi),
            })
        }
        Err(e) => HttpResponse::InternalServerError().json(DecodeResponse {
            status: "error".to_string(),
            function_name: None,
            arguments: None,
            message: Some("Error al decodificar los datos de llamada".to_string()),
            details: Some(e.to_string()),
            abi: None,
        }),
    }
}

// Manejador para la ruta /analysis
async fn analysis_handler(req: web::Json<AnalysisRequest>) -> impl Responder {
    let deepseek_api_key = match env::var("DEEPSEEK_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return HttpResponse::InternalServerError().json(AnalysisResponse {
                status: "error".to_string(),
                function_name: None,
                arguments: None,
                risk_level: None,
                explanation: None,
                message: Some("DEEPSEEK_API_KEY no configurada".to_string()),
                details: Some("Asegúrate de configurar la variable de entorno DEEPSEEK_API_KEY en tu archivo .env".to_string()),
            });
        }
    };

    // Parse contract address
    let contract_address = match req.contract_address.parse::<Address>() {
        Ok(addr) => addr,
        Err(e) => {
            return HttpResponse::BadRequest().json(AnalysisResponse {
                status: "error".to_string(),
                function_name: None,
                arguments: None,
                risk_level: None,
                explanation: None,
                message: Some(format!("Dirección de contrato inválida: {}", e)),
                details: None,
            });
        }
    };

    // Get or fetch ABI
    let (contract, _abi) = match get_or_fetch_abi(&contract_address).await {
        Ok((c, a)) => (c, a),
        Err(e) => {
            return HttpResponse::InternalServerError().json(AnalysisResponse {
                status: "error".to_string(),
                function_name: None,
                arguments: None,
                risk_level: None,
                explanation: None,
                message: Some("Error al obtener o cargar el ABI".to_string()),
                details: Some(e.to_string()),
            });
        }
    };

    // Decode function call
    let (function_name, arguments) = match decode_function_call(&contract, &req.call_data) {
        Ok((name, args)) => {
            let args_str: Vec<String> = args.into_iter().map(|arg| format!("{:?}", arg)).collect();
            (name, args_str)
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(AnalysisResponse {
                status: "error".to_string(),
                function_name: None,
                arguments: None,
                risk_level: None,
                explanation: None,
                message: Some("Error al decodificar los datos de llamada".to_string()),
                details: Some(e.to_string()),
            });
        }
    };

    // Cargar configuración del prompt
    let prompt_config = match load_prompt_config() {
        Ok(config) => config,
        Err(e) => {
            return HttpResponse::InternalServerError().json(AnalysisResponse {
                status: "error".to_string(),
                function_name: Some(function_name),
                arguments: Some(arguments),
                risk_level: None,
                explanation: None,
                message: Some("Error al cargar la configuración del prompt".to_string()),
                details: Some(e.to_string()),
            });
        }
    };

    let api_url = match Url::parse("https://api.deepseek.com/chat/completions") {
        Ok(url) => url,
        Err(e) => {
            return HttpResponse::InternalServerError().json(AnalysisResponse {
                status: "error".to_string(),
                function_name: Some(function_name),
                arguments: Some(arguments),
                risk_level: None,
                explanation: None,
                message: Some("Error interno al construir la URL de la API".to_string()),
                details: Some(e.to_string()),
            });
        }
    };

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", deepseek_api_key)).unwrap(),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    // Construct the prompt for the LLM using the config
    let prompt = prompt_config
        .user_prompt_template
        .replace("{contract_address}", &req.contract_address)
        .replace("{function_name}", &function_name)
        .replace("{arguments}", &format!("{:?}", arguments));

    let body = json!({
        "model": prompt_config.model_settings.model,
        "messages": [
            {"role": "system", "content": prompt_config.system_message},
            {"role": "user", "content": prompt}
        ],
        "stream": prompt_config.model_settings.stream
    });

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await;

    match response {
        Ok(res) => {
            let status = res.status();
            let full_response: Result<Value, _> = res.json().await;

            match full_response {
                Ok(json_response) => {
                    let content = json_response["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("");

                    let risk_level = content
                        .lines()
                        .find(|line| {
                            line.starts_with(&prompt_config.response_format.risk_level_prefix)
                        })
                        .and_then(|line| line.split(":").nth(1))
                        .map(|s| s.trim().to_string());

                    let explanation = content
                        .lines()
                        .find(|line| {
                            line.starts_with(&prompt_config.response_format.explanation_prefix)
                        })
                        .and_then(|line| line.split(":").nth(1))
                        .map(|s| s.trim().to_string())
                        .or_else(|| {
                            // Si no encuentra EXPLANATION:, busca líneas después de EXPLANATION:
                            let mut found_explanation = false;
                            let explanation_lines: Vec<&str> = content
                                .lines()
                                .filter(|line| {
                                    if line.starts_with(
                                        &prompt_config.response_format.explanation_prefix,
                                    ) {
                                        found_explanation = true;
                                        false // No incluir esta línea
                                    } else {
                                        found_explanation
                                    }
                                })
                                .collect();

                            if !explanation_lines.is_empty() {
                                Some(explanation_lines.join("\n"))
                            } else {
                                None
                            }
                        });

                    if status.is_success() {
                        HttpResponse::Ok().json(AnalysisResponse {
                            status: "success".to_string(),
                            function_name: Some(function_name),
                            arguments: Some(arguments),
                            risk_level,
                            explanation,
                            message: Some("Análisis de riesgo completado".to_string()),
                            details: None,
                        })
                    } else {
                        HttpResponse::InternalServerError().json(AnalysisResponse {
                            status: "error".to_string(),
                            function_name: Some(function_name),
                            arguments: Some(arguments),
                            risk_level: None,
                            explanation: None,
                            message: Some(format!(
                                "Error en la API de DeepSeek (HTTP status: {})",
                                status
                            )),
                            details: Some(json_response.to_string()),
                        })
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(AnalysisResponse {
                    status: "error".to_string(),
                    function_name: Some(function_name),
                    arguments: Some(arguments),
                    risk_level: None,
                    explanation: None,
                    message: Some("Error al parsear la respuesta JSON de DeepSeek".to_string()),
                    details: Some(e.to_string()),
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(AnalysisResponse {
            status: "error".to_string(),
            function_name: Some(function_name),
            arguments: Some(arguments),
            risk_level: None,
            explanation: None,
            message: Some("Error al llamar a la API de DeepSeek".to_string()),
            details: Some(e.to_string()),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let server_address = "127.0.0.1:8080";

    println!("🚀 Servidor web iniciando en http://{}", server_address);

    HttpServer::new(|| {
        App::new()
            .route("/decode", web::post().to(decode_handler))
            .route("/analysis", web::post().to(analysis_handler))
    })
    .bind(server_address)?
    .run()
    .await
}
