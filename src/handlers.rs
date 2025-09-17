use actix_web::{web, HttpResponse, Responder};
use ethers::types::Address;
use log::{error, info, warn};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde_json::{json, Value};
use std::env;
use url::Url;

use crate::abi::get_or_fetch_abi;
use crate::config::load_prompt_config;
use crate::decode::decode_function_call;
use crate::{AnalysisRequest, AnalysisResponse, DecodeRequest, DecodeResponse};

pub async fn decode_handler(req: web::Json<DecodeRequest>) -> impl Responder {
    info!(
        "📥 Petición recibida en /decode - Contrato: {}",
        req.contract_address
    );

    let contract_address_result = req.contract_address.parse::<Address>();
    let contract_address = match contract_address_result {
        Ok(addr) => addr,
        Err(e) => {
            warn!(
                "❌ Dirección de contrato inválida: {} - Error: {}",
                req.contract_address, e
            );
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
            error!("❌ Error al obtener ABI para {}: {}", contract_address, e);
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
            info!(
                "✅ Decodificación exitosa - Función: {}, Argumentos: {:?}",
                name, args_str
            );
            HttpResponse::Ok().json(DecodeResponse {
                status: "success".to_string(),
                function_name: Some(name),
                arguments: Some(args_str),
                message: None,
                details: None,
                abi: Some(abi),
            })
        }
        Err(e) => {
            error!("❌ Error al decodificar call data: {}", e);
            HttpResponse::InternalServerError().json(DecodeResponse {
                status: "error".to_string(),
                function_name: None,
                arguments: None,
                message: Some("Error al decodificar los datos de llamada".to_string()),
                details: Some(e.to_string()),
                abi: None,
            })
        }
    }
}

pub async fn analysis_handler(req: web::Json<AnalysisRequest>) -> impl Responder {
    info!(
        "📥 Petición recibida en /analysis - Contrato: {}",
        req.contract_address
    );

    let deepseek_api_key = match env::var("DEEPSEEK_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!(
                "❌ DEEPSEEK_API_KEY no configurada para análisis de contrato: {}",
                req.contract_address
            );
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
            warn!(
                "❌ Dirección de contrato inválida en análisis: {} - Error: {}",
                req.contract_address, e
            );
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
            error!(
                "❌ Error al obtener ABI para análisis de {}: {}",
                contract_address, e
            );
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
            error!("❌ Error al decodificar call data en análisis: {}", e);
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
            error!("❌ Error al cargar configuración del prompt: {}", e);
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
            error!("❌ Error al construir URL de API DeepSeek: {}", e);
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

    info!(
        "📤 Enviando solicitud a DeepSeek API - Función: {}",
        function_name
    );

    let response = client
        .post(api_url)
        .headers(headers)
        .json(&body)
        .send()
        .await;

    match response {
        Ok(res) => {
            let status = res.status();
            info!("📥 Respuesta de DeepSeek - Status: {}", status);
            let full_response: Result<Value, _> = res.json().await;

            match full_response {
                Ok(json_response) => {
                    let content = json_response["choices"][0]["message"]["content"]
                        .as_str()
                        .unwrap_or("");

                    // Log del contenido completo para depuración
                    info!("📄 Contenido completo de la respuesta LLM: {}", content);

                    let risk_level = content
                        .lines()
                        .find(|line| {
                            line.starts_with(&prompt_config.response_format.risk_level_prefix)
                        })
                        .and_then(|line| line.split(":").nth(1))
                        .map(|s| s.trim().to_string());

                    let explanation = if let Some(start) =
                        content.find(&prompt_config.response_format.explanation_prefix)
                    {
                        let after_prefix =
                            start + prompt_config.response_format.explanation_prefix.len();
                        if content[after_prefix..].starts_with(':') {
                            Some(content[(after_prefix + 1)..].trim().to_string())
                        } else {
                            Some(content[after_prefix..].trim().to_string())
                        }
                    } else {
                        None
                    };

                    if status.is_success() {
                        info!("✅ Análisis completado exitosamente - Función: {}, Nivel de riesgo: {:?}", function_name, risk_level);
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
                        error!(
                            "❌ Error en API DeepSeek - Status: {}, Respuesta: {}",
                            status, json_response
                        );
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
                Err(e) => {
                    error!("❌ Error al parsear JSON de DeepSeek: {}", e);
                    HttpResponse::InternalServerError().json(AnalysisResponse {
                        status: "error".to_string(),
                        function_name: Some(function_name),
                        arguments: Some(arguments),
                        risk_level: None,
                        explanation: None,
                        message: Some("Error al parsear la respuesta JSON de DeepSeek".to_string()),
                        details: Some(e.to_string()),
                    })
                }
            }
        }
        Err(e) => {
            error!("❌ Error al llamar a API DeepSeek: {}", e);
            HttpResponse::InternalServerError().json(AnalysisResponse {
                status: "error".to_string(),
                function_name: Some(function_name),
                arguments: Some(arguments),
                risk_level: None,
                explanation: None,
                message: Some("Error al llamar a la API de DeepSeek".to_string()),
                details: Some(e.to_string()),
            })
        }
    }
}
