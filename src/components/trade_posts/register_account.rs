use crate::components::common::spinner::LoadingSpinner;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
}

#[component]
pub fn RegisterAccount() -> impl IntoView {
    let (result, set_result) = signal("".to_string());
    let register_action: Action<(), (), LocalStorage> = Action::new_unsync(move |_| async move {
        let response = invoke_without_args("register_account").await;
        match response.as_string() {
            Some(res) => set_result.set(res),
            None => set_result.set("Registration failed!".to_string()),
        }
    });

    let pending = register_action.pending(); // ReadSignal<bool>

    view! {
            <div class="max-w-md mx-auto p-6 bg-gray-50 dark:bg-gray-800 rounded-lg shadow-md">
            <p class="text-gray-900 dark:text-gray-100 mb-6">"If you have not register the account, please resister"</p>
                <h2 class="text-2xl font-bold text-gray-900 dark:text-gray-100 mb-6">Register Account</h2>
                <button
                    on:click=move |_| { register_action.dispatch(()); }
                    class="w-full px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-semibold rounded-md shadow-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                    Register
                </button>
                <br/>
                <br/>
                <p>{move || pending.get().then_some( view! { <LoadingSpinner /> }.into_any())}</p>

                <p class="mt-4 text-sm text-gray-700 dark:text-gray-300">{result}</p>
        </div>
    }
}
