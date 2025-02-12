use serde_wasm_bindgen::from_value; // Import this!
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::components::navigation::nav::Nav;
use crate::components::common::spinner::LoadingSpinner;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Pool {
    pub amounts: Vec<String>,
    pub pool_kind: String,
    pub shares_total_supply: String,
    pub token_account_ids: Vec<String>,
    pub total_fee: u32,
}

#[component]
pub fn GetPoolsDetails() -> impl IntoView {
    let (pools, set_pools) = signal(Vec::<Pool>::new());

    let fetch_pools: Action<(), (), LocalStorage> = Action::new_unsync(move |_| async move {

        // set_loading.set(true);
        let result = invoke_without_args("get_pools").await;
     
        // gloo::console::log!("resulut", result.clone());


        web_sys::console::log_1(&result.clone().into()); // Log JsValue for debugging

        // Convert JsValue to Vec<Pool> using serde-wasm-bindgen
        match from_value::<Vec<Pool>>(result) {
            Ok(parsed_pools) => {
                web_sys::console::log_1(&format!("Parsed pools: {:?}", parsed_pools).into());
                set_pools.set(parsed_pools);
            }
            Err(e) => {
                web_sys::console::log_1(&format!("Failed to parse pools: {}", e).into());
            }
        }

        
    });

    let pending = fetch_pools.pending(); // ReadSignal<bool>


    view! {
        <>
        <Nav/>
        <div class="p-6 bg-gray-100 dark:bg-gray-900 min-h-screen">
        <div class="max-w-2xl mx-auto bg-white dark:bg-gray-800 shadow-lg rounded-2xl p-6">
            <h2 class="text-xl font-bold text-gray-800 dark:text-white dark:text-gray-100 mb-4">Liquidity Pools</h2>

            <button
                on:click=move |_| {fetch_pools.dispatch(());}
                class="w-full bg-blue-600 hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600 dark:text-white font-semibold py-2 px-4 rounded-lg transition duration-300 cursor-pointer"
            >
                "Fetch Pools"
            </button>

            <p>{move || pending.get().then_some( view! { <LoadingSpinner /> }.into_any())}</p>


            <ul class="mt-4 space-y-4">
                { move || pools.get().iter().map(|pool| view! {
                    <li class="p-4 border rounded-lg shadow-sm bg-gray-50 dark:bg-gray-700">
                        <strong class="block text-lg font-semibold text-gray-900 dark:text-gray-200">
                            {pool.pool_kind.clone()}
                        </strong>
                        <p class="text-gray-600 dark:text-gray-400">Tokens: {pool.token_account_ids.join(", ")}</p>
                        <p class="text-gray-600 dark:text-gray-400">Total Fee: {pool.total_fee}</p>
                    </li>
                }).collect_view()}
            </ul>
        </div>
    </div>
    </>
    }
}
