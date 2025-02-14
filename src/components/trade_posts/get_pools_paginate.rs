use crate::components::common::spinner::LoadingSpinner;
use crate::components::navigation::nav::Nav;
use leptos::html;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_wasm_bindgen::from_value; // Import this!
use wasm_bindgen::prelude::*;
use leptos::ev;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct FetchPoolsArgs {
    index: u64,
    limit: u64,
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
pub fn GetPoolsPaginate() -> impl IntoView {
    let (pools, set_pools) = signal(Vec::<Pool>::new());

    let (page, set_page) = signal(1); // Current page
    let (page_size, set_page_size) = signal(10); // Number of items per page
    let input_element_page = NodeRef::<html::Input>::new();
    let input_element_page_size = NodeRef::<html::Input>::new();

    let fetch_pools: Action<(), (), LocalStorage> = Action::new_unsync(move|_: &()| {
        async move {
            let args = serde_wasm_bindgen::to_value(&FetchPoolsArgs {
                index: (page.get() - 1) * page_size.get(),
                limit: page_size.get(),
            })
            .unwrap();

            web_sys::console::log_1(&args.clone().into()); // Log JsValue for debugging


            let result = invoke("get_pools_paginate", args).await;

            web_sys::console::log_1(&result.clone().into()); // Log JsValue for debugging

            match from_value::<Vec<Pool>>(result) {
                Ok(parsed_pools) => {
                    web_sys::console::log_1(&format!("Parsed pools: {:?}", parsed_pools).into());
                    set_pools.set(parsed_pools);
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Failed to parse pools: {}", e).into());
                }
            }
        }
    });

    // Go to a specific page
    let go_to_page = move |new_page:u64| {
        if new_page >= 1 {
            set_page.set(new_page);
            fetch_pools.dispatch(());
        }
    };

    // Update page number from form
    let update_page = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let new_page = input_element_page
            .get()
            .unwrap()
            .value()
            .parse::<u64>()
            .unwrap_or(1);
        go_to_page(new_page);
    };

    // Update page size from form
    let update_page_size = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let new_page_size = input_element_page_size
            .get()
            .unwrap()
            .value()
            .parse::<u64>()
            .unwrap_or(10);
        set_page_size.set(new_page_size);
        fetch_pools.dispatch(());
    };

    let pending = fetch_pools.pending(); // ReadSignal<bool>

    view! {
        <>
            <Nav />
            <div class="p-6 bg-gray-100 dark:bg-gray-900 min-h-screen">
                <div class="max-w-2xl mx-auto bg-white dark:bg-gray-800 shadow-lg rounded-2xl p-6">
                    <h2 class="text-xl font-bold text-gray-800 dark:text-white dark:text-gray-100 mb-4">
                        Liquidity Pools
                    </h2>

                    <button
                        on:click=move |_| { fetch_pools.dispatch(()); }
                        class="w-full bg-blue-600 hover:bg-blue-700 dark:bg-blue-500 dark:hover:bg-blue-600 dark:text-white font-semibold py-2 px-4 rounded-lg transition duration-300 cursor-pointer"
                    >
                        "Fetch Pools"
                    </button>

                    <p>{move || pending.get().then_some(view! { <LoadingSpinner /> }.into_any())}</p>

                    <ul class="mt-4 space-y-4">
                        {move || pools.get().iter().map(|pool| view! {
                            <li class="p-4 border rounded-lg shadow-sm bg-gray-50 dark:bg-gray-700">
                                <strong class="block text-lg font-semibold text-gray-900 dark:text-gray-200">
                                    {pool.pool_kind.clone()}
                                </strong>
                                <p class="text-gray-600 dark:text-gray-400">Tokens: {pool.token_account_ids.join(", ")}</p>
                                <p class="text-gray-600 dark:text-gray-400">Total Fee: {pool.total_fee}</p>
                            </li>
                        }).collect_view()}
                    </ul>

                    // Pagination controls
                    {move || if !pools.get().is_empty() {
                        view! {
                            <div class="flex items-center justify-between mt-4">
                                <button
                                    class="px-4 py-2 bg-blue-500 text-white rounded disabled:opacity-50"
                                    on:click=move |_| go_to_page(page() - 1)
                                    disabled=move || page() <= 1
                                >
                                    "Previous"
                                </button>
                                <span class="text-gray-700">
                                    "Page " {page}
                                </span>
                                <button
                                    class="px-4 py-2 bg-blue-500 text-white rounded"
                                    on:click=move |_| go_to_page(page() + 1)
                                >
                                    "Next"
                                </button>
                            </div>

                            <div class="flex flex-col space-y-2 sm:flex-row sm:space-y-0 sm:space-x-4 mt-4">
                                // Page number form
                                <form on:submit=update_page class="w-full sm:w-auto">
                                    <div class="flex items-center space-x-2">
                                        <label class="w-24 text-gray-700 font-medium">
                                            "Page Number:"
                                        </label>
                                        <input
                                            type="number"
                                            class="w-full p-2 border rounded sm:w-auto"
                                            node_ref=input_element_page
                                            value=move || page().to_string()
                                        />
                                        <button type="submit" class="px-4 py-2 bg-green-500 text-white rounded">
                                            "Update"
                                        </button>
                                    </div>
                                </form>

                                // Page size form
                                <form on:submit=update_page_size class="w-full sm:w-auto">
                                    <div class="flex items-center space-x-2">
                                        <label class="w-24 text-gray-700 font-medium">
                                            "Page Size:"
                                        </label>
                                        <input
                                            type="number"
                                            class="w-full p-2 border rounded sm:w-auto"
                                            node_ref=input_element_page_size
                                            value=move || page_size().to_string()
                                        />
                                        <button type="submit" class="px-4 py-2 bg-green-500 text-white rounded">
                                            "Update"
                                        </button>
                                    </div>
                                </form>
                            </div>
                        }.into_any()
                    } else {
                        view! {}.into_any()
                    }}
                </div>
            </div>
        </>
    }
}