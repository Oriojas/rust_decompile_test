use serde_json;
use std::fs;

use crate::PromptConfig;

pub fn load_prompt_config() -> Result<PromptConfig, Box<dyn std::error::Error>> {
    let config_path = "src/prompt_config.json";
    let config_content = fs::read_to_string(config_path)?;
    let config: PromptConfig = serde_json::from_str(&config_content)?;
    Ok(config)
}
