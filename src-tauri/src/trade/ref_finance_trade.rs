use crate::trade::trade_history::{Trade, TradeSwap};
use near_api::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_store::StoreExt;
use serde_json::Value;

#[derive(Deserialize, Debug, Serialize)]
pub struct Pool {
    pub amounts: Vec<String>,
    pub pool_kind: String,
    pub shares_total_supply: String,
    pub token_account_ids: Vec<String>,
    pub total_fee: u32,
}

#[tauri::command]
pub async fn get_pools() -> Result<Vec<Pool>, String> {
    let network = NetworkConfig::testnet();

    let contract_id: AccountId = "ref-finance.testnet".parse().unwrap();
    let contract = Contract(contract_id.clone());

    let args = json!({
        "from_index": 0,
        "limit": 10
    });

    let view_call_result: Data<Vec<Pool>> = contract
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
pub async fn get_pool(pool_id: u64) -> Result<Pool, String> {
    let network = NetworkConfig::testnet();

    let contract_id: AccountId = "ref-finance.testnet".parse().unwrap();
    let contract = Contract(contract_id.clone());

    let args = json!({
        "pool_id": pool_id,
    });

    let view_call_result: Data<Pool> = contract
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

#[tauri::command(rename_all = "snake_case")]
pub async fn swap(app: tauri::AppHandle, pool_id: u64, token_in: String, amount_in: String, token_out: String, ) -> Result<String, String> {
    let store = app.store("account_details").map_err(|e| e.to_string())?;
    let account_id_value = store.get("account_id").unwrap();
    let account_id = match account_id_value {
        Value::String(s) => s,  
        _ => account_id_value.to_string(),  
    };
    
    let seed_value = store.get("seed").unwrap();

    let seed = match seed_value {
        Value::String(s) => s,  
        _ => seed_value.to_string(),  
    };
    println!("seed: {}", seed);
    let network = NetworkConfig::testnet();
    let contract_id: AccountId = "ref-finance.testnet".parse().unwrap();
    let contract = Contract(contract_id.clone());
    let args = json!({"actions": [{"pool_id": pool_id, "token_in": token_in, "amount_in": amount_in, "token_out": token_out, "min_amount_out": "1"}]});
    let account_id: AccountId = account_id.parse().unwrap();
    let seed_phrase = Signer::from_seed_phrase(&seed, None).unwrap(); 
    let signer = Signer::new(seed_phrase).unwrap();
    let function_call_result = contract
        .call_function("swap", args)
        .unwrap()
        .transaction()
        .deposit(NearToken::from_near(1))
        .with_signer(account_id.clone(), signer.clone()) 
        .send_to(&network)
        .await
        .unwrap();
    let store = app.store("storeswap.json").map_err(|e| e.to_string())?;

    let mut history: Vec<TradeSwap> = store
        .get("trade_history")
        .unwrap_or(json!([]))
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|v| serde_json::from_value(v.clone()).unwrap())
        .collect();

    let new_trade = TradeSwap {
        pool_id: pool_id,
        token_in: token_in,
        amount_in: amount_in,
        token_out: token_out,
        timestamp: chrono::Utc::now().timestamp(),
    };

    history.push(new_trade);

    store.set("trade_history", json!(history));
    store.save().map_err(|e| e.to_string())?;

    
    Ok(format!("{:?}", function_call_result))
}


#[tauri::command(rename_all = "snake_case")]
pub async fn register_account(app: tauri::AppHandle ) -> Result<String, String> {
    let store = app.store("account_details").map_err(|e| e.to_string())?;
    let account_id_value = store.get("account_id").unwrap();
    let account_id = match account_id_value {
        Value::String(s) => s,  
        _ => account_id_value.to_string(),  
    };
    
    let seed_value = store.get("seed").unwrap();

    let seed = match seed_value {
        Value::String(s) => s,  
        _ => seed_value.to_string(),  
    };
    println!("seed: {}", seed);
    let network = NetworkConfig::testnet();
    let contract_id: AccountId = "ref-finance.testnet".parse().unwrap();
    let contract = Contract(contract_id.clone());
    let account_id: AccountId = account_id.parse().unwrap();
    let seed_phrase = Signer::from_seed_phrase(&seed, None).unwrap(); 
    let signer = Signer::new(seed_phrase).unwrap();
     let args = json!({});
    let function_call_result = contract
        .call_function("storage_deposit", args)
        .unwrap()
        .transaction()
        .deposit(NearToken::from_near(1))
        .with_signer(account_id.clone(), signer.clone()) 
        .send_to(&network)
        .await
        .unwrap();
    Ok(format!("{:?}", function_call_result))
}


#[tauri::command]
pub async fn get_swap_history(app: tauri::AppHandle) -> Result<String, String> {
    let store = app.store("storeswap.json").map_err(|e| e.to_string())?;
    let history = store.get("trade_history").unwrap_or(json!([]));

    // Convert JSON to a String
    Ok(history.to_string())
}
