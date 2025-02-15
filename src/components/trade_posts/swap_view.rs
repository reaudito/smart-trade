use crate::components::navigation::nav::Nav;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

// Update the invoke binding to accept arguments
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SwapArgs {
    pool_id: u64,
    token_in: String,
    amount_in: String,
    token_out: String,
    min_amount_out: String,
}

#[component]
pub fn SwapInterface() -> impl IntoView {
    let (token_in, set_token_in) = signal("".to_string());
    let (amount_in, set_amount_in) = signal("".to_string());
    let (token_out, set_token_out) = signal("".to_string());
    let (min_amount_out, set_min_amount_out) = signal("".to_string());
    let (result, set_result) = signal("".to_string());

    let swap_action: Action<(), (), LocalStorage> = Action::new_unsync(move |_| {
        let token_in = token_in.get();
        let amount_in = amount_in.get();
        let token_out = token_out.get();
        let min_amount_out = min_amount_out.get();

        async move {
            let args = to_value(&SwapArgs {
                pool_id: 0, // Replace with actual pool ID
                token_in: token_in.clone(),
                amount_in: amount_in.clone(),
                token_out: token_out.clone(),
                min_amount_out: min_amount_out.clone(),
            })
            .unwrap();

            // Call the Tauri backend
            let result = invoke("swap", args).await;

            // Handle the result
            match result.as_string() {
                Some(msg) => set_result.set(msg),
                None => set_result.set("Swap failed!".to_string()),
            }
        }
    });

    view! {
        <>
        <Nav/>
        <div class="min-h-screen bg-white dark:bg-gray-900">
        <br/>
        <br/>
        <div class="max-w-md mx-auto p-6 bg-gray-50 dark:bg-gray-800 rounded-lg shadow-md">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6">Swap Tokens</h2>
            <form on:submit=move |ev| { ev.prevent_default(); swap_action.dispatch(()); }>
                <div class="mb-4">
                    <label for="token_in" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        Token In:
                    </label>
                    <input
                        type="text"
                        id="token_in"
                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-gray-100"
                        value=token_in
                        on:input=move |ev| set_token_in.set(event_target_value(&ev))
                    />
                </div>
                <div class="mb-4">
                    <label for="amount_in" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        Amount In:
                    </label>
                    <input
                        type="text"
                        id="amount_in"
                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-gray-100"
                        value=amount_in
                        on:input=move |ev| set_amount_in.set(event_target_value(&ev))
                    />
                </div>
                <div class="mb-4">
                    <label for="token_out" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        Token Out:
                    </label>
                    <input
                        type="text"
                        id="token_out"
                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-gray-100"
                        value=token_out
                        on:input=move |ev| set_token_out.set(event_target_value(&ev))
                    />
                </div>
                <div class="mb-6">
                    <label for="min_amount_out" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        Minimum Amount Out:
                    </label>
                    <input
                        type="text"
                        id="min_amount_out"
                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-gray-100"
                        value=min_amount_out
                        on:input=move |ev| set_min_amount_out.set(event_target_value(&ev))
                    />
                </div>
                <button
                    type="submit"
                    class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-md shadow-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                    Swap
                </button>
            </form>
            <p class="mt-4 text-sm text-gray-700 dark:text-gray-300">{result}</p>
        </div>
        </div>
        </>
    }
}
