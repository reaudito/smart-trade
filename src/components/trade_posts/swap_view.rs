use crate::components::common::spinner::LoadingSpinner;
use crate::components::navigation::nav::Nav;
use crate::components::trade_posts::get_pool_details_props::GetPoolDetailsProps;
use crate::components::trade_posts::register_account::RegisterAccount;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

// Update the invoke binding to accept arguments
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct SwapArgs {
    pool_id: u64,
    token_in: String,
    amount_in: String,
    token_out: String,
}

#[derive(Serialize, Deserialize)]
struct HistroyArg {
    content: String,
}

#[component]
pub fn SwapInterface() -> impl IntoView {
    let (pool_id, set_pool_id) = signal(0u64);
    let (token_in, set_token_in) = signal("".to_string());
    let (amount_in, set_amount_in) = signal("".to_string());
    let (token_out, set_token_out) = signal("".to_string());
    let (result, set_result) = signal("".to_string());
    let (result_ai, set_result_ai) = signal("".to_string());

    let swap_action: Action<(), (), LocalStorage> = Action::new_unsync(move |_| {
        let token_in = token_in.get();
        let amount_in = amount_in.get();
        let token_out = token_out.get();
        let pool_id = pool_id.get();
        async move {
            let args = to_value(&SwapArgs {
                pool_id: pool_id, // Replace with actual pool ID
                token_in: token_in.clone(),
                amount_in: amount_in.clone(),
                token_out: token_out.clone(),
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

    let ask_ai_action: Action<(), (), LocalStorage> = Action::new_unsync(move |_| {
        let token_in = token_in.get();
        let amount_in = amount_in.get();
        let token_out = token_out.get();
        let pool_id = pool_id.get();
        async move {
            let history = invoke_without_args("get_swap_history")
                .await
                .as_string()
                .unwrap_or("No history found".to_string()); // Replace this with actual trade history
            let current_trade = format!(
                "Current trade is pool_id: {}, token_in: {}, amount_id: {}, token_out: {}",
                pool_id,
                token_in.clone(),
                amount_in.clone(),
                token_out.clone()
            );

            let message = format!(
                "Should I trade? {}, swap history {}",
                current_trade, history
            );

            let args = to_value(&HistroyArg { content: message }).unwrap();
            let ai_response = invoke("ask_ai", args).await;
            match ai_response.as_string() {
                Some(res) => set_result_ai.set(res),
                None => set_result_ai.set("AI decision failed!".to_string()),
            }
        }
    });

    let pending = swap_action.pending();
    let pending_ask_ai = ask_ai_action.pending();

    view! {
        <>
        <Nav/>
        <div class="min-h-screen bg-white dark:bg-gray-900">
        <br/>
        <br/>
        <RegisterAccount/>
        <div class="max-w-md mx-auto p-6 bg-gray-50 dark:bg-gray-800 rounded-lg shadow-md">
            <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6">Swap Tokens</h2>
            <form on:submit=move |ev| { ev.prevent_default(); swap_action.dispatch(()); }>
            <div class="mb-4">
                    <label for="pool_id" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                        Pool ID:
                    </label>
                    <input
                        type="number"
                        id="pool_id"
                        class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-gray-100"
                        value=pool_id
                        on:input=move |ev| set_pool_id.set(event_target_value(&ev).parse().unwrap_or(0))
                    />
                </div>
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
                <button
                    type="submit"
                    class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-md shadow-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                    Swap
                </button>
            </form>
            <br/>
            <br/>
            <p>{move || pending.get().then_some( view! { <LoadingSpinner /> }.into_any())}</p>

            <p class="mt-4 text-sm text-gray-700 dark:text-gray-300">{result}</p>
       

        <button
            on:click=move |_| {ask_ai_action.dispatch(()); }
            class="w-full px-4 py-2 mt-4 bg-green-600 hover:bg-green-700 text-white font-semibold rounded-md shadow-md focus:outline-none focus:ring-2 focus:ring-green-500"
        >
            Ask AI if you should Trade
        </button>
        <br/>
        <br/>
        <p>{move || pending_ask_ai.get().then_some( view! { <LoadingSpinner /> }.into_any())}</p>

        {move || {
                let html_content = result_ai
                    .get()
                    .replace("\n", "<br>")
                    .replace("<think>", r#"<think><span class="italic text-sm">"#)
                    .replace("</think>", "</span></think>");
                view! { <div class="dark:text-white" inner_html=html_content></div> }.into_any()
            }
        }

        </div>


        {move ||
            view! { <GetPoolDetailsProps pool_id=pool_id.get() /> }.into_any()
        }
        </div>
        </>
    }
}
