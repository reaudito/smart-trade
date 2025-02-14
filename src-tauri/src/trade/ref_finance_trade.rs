use near_api::*;
use serde_json::json;
use tauri_plugin_store::StoreExt;
use crate::trade::trade_history::Trade;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug, Serialize)]
pub struct Pool {
    pub amounts: Vec<String>,
    pub pool_kind: String,
    pub shares_total_supply: String,
    pub token_account_ids: Vec<String>,
    pub total_fee: u32,
}

#[tauri::command]
pub async fn get_pools() ->  Result<Vec<Pool>, String> {

    let network = NetworkConfig::testnet();

    let contract_id: AccountId = "ref-finance.testnet".parse().unwrap();
    let contract = Contract(contract_id.clone());

    let args = json!({
        "from_index": 0,
        "limit": 10
    });

    let view_call_result:Data<Vec<Pool>>= contract
    .call_function("get_pools", args)
    .unwrap()
    .read_only()
    .fetch_from(&network)
    .await
    .unwrap();

    // println!("{:?}", view_call_result.data);

    Ok(view_call_result.data)


}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_pool(pool_id: u64) ->  Result<Pool, String> {

    let network = NetworkConfig::testnet();

    let contract_id: AccountId = "ref-finance.testnet".parse().unwrap();
    let contract = Contract(contract_id.clone());

    let args = json!({
        "pool_id": pool_id,
    });

    let view_call_result:Data<Pool>= contract
    .call_function("get_pool", args)
    .unwrap()
    .read_only()
    .fetch_from(&network)
    .await
    .unwrap();

    // println!("{:?}", view_call_result.data);

    Ok(view_call_result.data)


}


#[tauri::command(rename_all = "snake_case")]
pub async fn get_pools_paginate(from_index: u64, limit: u64) -> Result<Vec<Pool>, String> {
    let network = NetworkConfig::testnet();

    let contract_id: AccountId = "ref-finance.testnet".parse().unwrap();
    let contract = Contract(contract_id.clone());

    let args = json!({
        "from_index": from_index,
        "limit": limit
    });

    let view_call_result: Data<Vec<Pool>> = contract
        .call_function("get_pools", args)
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await
        .unwrap();

    Ok(view_call_result.data)
}

#[tauri::command]
pub async fn execute_trade(
    app: tauri::AppHandle,
    action: &str,
    symbol: &str,
    price: f64,
) -> Result<String, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;

    let mut history: Vec<Trade> = store
        .get("trade_history")
        .unwrap_or(json!([]))
        .as_array()
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
pub async fn get_trade_history(app: tauri::AppHandle) -> Result<String, String> {
    let store = app.store("store.json").map_err(|e| e.to_string())?;
    let history = store.get("trade_history").unwrap_or(json!([]));

    // Convert JSON to a String
    Ok(history.to_string())
}