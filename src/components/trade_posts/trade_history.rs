use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;
use crate::components::navigation::nav::Nav;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct TradeArgs<'a> {
    action: &'a str,
    symbol: &'a str,
    price: f64,
}

#[component]
pub fn MakeTrade() -> impl IntoView {
    let (history, set_history) = signal(Vec::<String>::new());

    let fetch_history: Action<(), (), LocalStorage> =
        Action::new_unsync(move |_: &()| async move {
            let trades = invoke_without_args("get_trade_history").await;

            gloo::console::log!(trades.clone().as_string());

            let trades_str = trades.as_string().unwrap_or_default(); // Get the JSON string
            let trades_json: Value = serde_json::from_str(&trades_str).unwrap_or(json!([])); // Parse string to JSON

            let trades_vec: Vec<String> = trades_json
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|t| format!("{} {} at {}", t["action"], t["symbol"], t["price"]))
                .collect();

            gloo::console::log!(trades_vec.clone());

            set_history.set(trades_vec);
        });

    let execute_trade: Action<String, (), LocalStorage> = Action::new_unsync(|input: &String| {
        let call_data = input.to_owned();
        async move {
            let args = serde_wasm_bindgen::to_value(&TradeArgs {
                action: &call_data.to_owned(),
                symbol: &"BTCUSDT".to_string(),
                price: 50000.0,
            })
            .unwrap();
            let response = invoke("execute_trade", args).await;

            gloo::console::log!(response.as_string());
        }
    });

    view! {
        <div>
            <Nav />
            <div class="p-4 bg-white text-black dark:bg-gray-900 dark:text-white min-h-screen">
                <h1 class="text-2xl font-bold mb-4">AI Trading Bot</h1>

                <div class="flex gap-2">
                    <button
                        class="w-32 h-10 bg-blue-500 hover:bg-blue-600 text-white font-medium rounded-lg transition"
                        on:click=move |_| {
                            fetch_history.dispatch(());
                        }
                    >
                        >
                        Load History
                    </button>

                    <button
                        class="w-32 h-10 bg-green-500 hover:bg-green-600 text-white font-medium rounded-lg transition"
                        on:click=move |_| {
                            execute_trade.dispatch("buy".to_string());
                        }
                    >
                        Buy
                    </button>

                    <button
                        class="w-32 h-10 bg-red-500 hover:bg-red-600 text-white font-medium rounded-lg transition"
                        on:click=move |_| {
                            execute_trade.dispatch("sell".to_string());
                        }
                    >
                        Sell
                    </button>
                </div>

                <ul class="mt-4 space-y-2">
                    {move || {
                        history
                            .get()
                            .iter()
                            .map(|trade| {
                                view! {
                                    <li class="bg-gray-100 dark:bg-gray-800 p-2 rounded-md">
                                        {trade.clone()}
                                    </li>
                                }
                                    .into_any()
                            })
                            .collect::<Vec<_>>()
                    }}
                </ul>
            </div>

        </div>
    }
}
