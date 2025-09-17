use ethabi::{Contract, Token};

pub fn decode_function_call(
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
