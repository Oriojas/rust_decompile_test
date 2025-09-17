use serde::{Deserialize, Serialize};
use serde_json::Value;

// Struct para la configuración del prompt
#[derive(Deserialize)]
pub struct PromptConfig {
    pub system_message: String,
    pub user_prompt_template: String,
    pub response_format: ResponseFormat,
    pub model_settings: ModelSettings,
}

#[derive(Deserialize)]
pub struct ResponseFormat {
    pub risk_level_prefix: String,
    pub explanation_prefix: String,
}

#[derive(Deserialize)]
pub struct ModelSettings {
    pub model: String,
    pub stream: bool,
}

// Struct para la petición JSON entrante del endpoint /decode
#[derive(Deserialize)]
pub struct DecodeRequest {
    pub contract_address: String,
    pub call_data: String,
}

// Struct para la respuesta JSON saliente del endpoint /decode
#[derive(Serialize)]
pub struct DecodeResponse {
    pub status: String, // "success" or "error"
    pub function_name: Option<String>,
    pub arguments: Option<Vec<String>>, // Represent arguments as strings for simplicity
    pub message: Option<String>,
    pub details: Option<String>, // For additional error info
    pub abi: Option<Value>,      // Include ABI in successful response for analysis endpoint
}

// Struct para la petición JSON entrante del endpoint /analysis
#[derive(Deserialize)]
pub struct AnalysisRequest {
    pub contract_address: String,
    pub call_data: String,
}

// Struct para la respuesta JSON saliente del endpoint /analysis
#[derive(Serialize)]
pub struct AnalysisResponse {
    pub status: String,                 // "success" or "error"
    pub function_name: Option<String>,  // Include decoded function name
    pub arguments: Option<Vec<String>>, // Include decoded arguments
    pub risk_level: Option<String>,     // e.g., "Low", "Medium", "High", "Caution", "Unknown"
    pub explanation: Option<String>,    // Explanation from the LLM
    pub message: Option<String>,
    pub details: Option<String>, // For additional error info
}

// Declaraciones de módulos
pub mod abi;
pub mod config;
pub mod decode;
pub mod handlers;
