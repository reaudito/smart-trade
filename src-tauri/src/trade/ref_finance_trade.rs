use near_api::*;
use serde_json::json;

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