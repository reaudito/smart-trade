use crate::components::common::spinner::LoadingSpinner;
use crate::components::navigation::nav::Nav;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

// Update the invoke binding to accept arguments
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Deserialize, Clone, Debug, Serialize)]
pub struct Pool {
    pub amounts: Vec<String>,
    pub pool_kind: String,
    pub shares_total_supply: String,
    pub token_account_ids: Vec<String>,
    pub total_fee: u32,
}

// Struct for command arguments
#[derive(Serialize)]
pub struct GetPoolArgs {
    pool_id: u64,
}
#[component]
pub fn GetPoolDetails() -> impl IntoView {
    let params = use_params_map();
    let pool_id = move || {
        params.with(|params| {
            params
                .get("pool_id")
                .and_then(|id| id.parse::<u64>().ok())
                .unwrap_or(0)
        })
    };
    let (pool, set_pool) = signal(None::<Pool>);

    let fetch_pool: Action<(), (), LocalStorage> = Action::new_unsync(move |_| async move {
        let args = GetPoolArgs {
            pool_id: pool_id(),
        };

        // Serialize arguments to JsValue
        let js_args = serde_wasm_bindgen::to_value(&args).unwrap();

        let result = invoke("get_pool", js_args).await;

        match from_value::<Pool>(result) {
            Ok(pool_data) => set_pool.set(Some(pool_data)),
            Err(e) => {
                web_sys::console::log_1(&format!("Error: {}", e).into());
                set_pool.set(None);
            }
        }
    });

    Effect::new(move |_| {
        fetch_pool.dispatch(());
      });

    let pending = fetch_pool.pending(); // ReadSignal<bool>

    view! {
        <>
            <Nav/>
            <div class="p-6 bg-gray-100 dark:bg-gray-900 min-h-screen">
                <div class="max-w-2xl mx-auto bg-white dark:bg-gray-800 shadow-lg rounded-2xl p-6">
                    <h2 class="text-xl font-bold text-gray-800 dark:text-white mb-4">Pool Details</h2>

                    <button
                        on:click=move |_| { fetch_pool.dispatch(()); }
                        class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-lg transition duration-300"
                    >
                        "Fetch Pool Details"
                    </button>

                    <p>{move || pending.get().then_some( view! { <LoadingSpinner /> }.into_any())}</p>

                    {move || pool.get().map(|pool| view! {
                        <div class="mt-4 p-4 border rounded-lg bg-gray-50 dark:bg-gray-700">
                            <h3 class="text-lg font-semibold dark:text-white">Pool {pool_id()}</h3>
                            <p class="dark:text-gray-300">Type: {pool.pool_kind}</p>
                            <p class="dark:text-gray-300">Total Fee: {pool.total_fee}</p>
                            <p class="dark:text-gray-300">Tokens: {pool.token_account_ids.join(", ")}</p>
                            <p class="dark:text-gray-300">Shares: {pool.shares_total_supply}</p>
                        </div>
                    }.into_any())}
                </div>
            </div>
        </>
    }
}
