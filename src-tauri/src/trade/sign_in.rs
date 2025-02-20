use near_api::*;
use serde_json::json;
use tauri_plugin_store::StoreExt;
use serde::{Deserialize, Serialize};
use tauri_plugin_store::JsonValue;

#[tauri::command(rename_all = "snake_case")]
pub async fn sign_in(app: tauri::AppHandle,seed: String, account_id: String) -> Result<String, String> {
    let store = app.store("account_details").map_err(|e| e.to_string())?;
    store.set("seed", seed);
    store.set("account_id", account_id.clone());
    store.save().map_err(|e| e.to_string())?;

    Ok(account_id)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_account_id(app: tauri::AppHandle) ->  Result<JsonValue, String> {
    let store = app.store("account_details").map_err(|e| e.to_string())?;
    let account_id = store.get("account_id").unwrap_or(json!("hello.near"));
    Ok(account_id)
}
