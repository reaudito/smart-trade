pub mod trade;

use crate::trade::trade_history::Trade;
use tauri_plugin_store::StoreExt;
use serde_json::{json, Value};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn improve_post(content: String) -> String {
    use ollama_rs::generation::completion::request::GenerationRequest;
    use ollama_rs::Ollama;

    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let model = "deepseek-r1:1.5b".to_string();

    if let Ok(res) = ollama
        .generate(GenerationRequest::new(model, content))
        .await
    {
        res.response
    } else {
        "Error generating improved post".to_string()
    }
}


#[tauri::command]
async fn execute_trade(
    app: tauri::AppHandle,
    action: &str,
    symbol: &str,
    price: f64,
) -> Result<String, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let mut history: Vec<Trade> = store.get("trade_history").unwrap_or(json!([])).as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|v| serde_json::from_value(v.clone()).unwrap())
        .collect();

    let new_trade = Trade {
        action: action.to_string(),
        symbol: symbol.to_string(),
        price,
        timestamp: chrono::Utc::now().timestamp(),
    };

    history.push(new_trade);

    store.set("trade_history", json!(history));
    store.save().map_err(|e| e.to_string())?;

    Ok(format!("Executed {} on {} at {}", action, symbol, price))
}

#[tauri::command]
async fn get_trade_history(app: tauri::AppHandle) -> Result<String, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let history = store.get("trade_history").unwrap_or(json!([]));

    // Convert JSON to a String
    Ok(history.to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, improve_post, execute_trade, get_trade_history])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
