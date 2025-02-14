pub mod trade;


use serde_json::{json, Value};
use tauri_plugin_store::StoreExt;

use crate::trade::ref_finance_trade::{get_pools, execute_trade, get_trade_history, get_pools_paginate};

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



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            improve_post,
            execute_trade,
            get_trade_history,
            get_pools,
            get_pools_paginate,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
