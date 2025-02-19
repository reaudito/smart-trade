use leptos::prelude::*;
use leptos_icons::*;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::from_value;
use reactive_stores::Store;
use crate::components::common::global_state::{GlobalState, GlobalStateStoreFields};


use leptos_use::{
    use_clipboard_with_options, use_permission, UseClipboardOptions, UseClipboardReturn,
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
}




#[component]
pub fn AccountNav() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    let account =state.account_state();

    let fetch_pool: Action<(), (), LocalStorage> = Action::new_unsync(move |_| async move {
    
        let result = invoke_without_args("get_account_id").await;

        match from_value::<String>(result) {
            Ok(account_id) => {
                web_sys::console::log_1(&format!("accound_id: {}", account_id.clone()).into());
                *account.write() = account_id;

            }
            Err(e) => {
                web_sys::console::log_1(&format!("Failed to parse pools: {}", e).into());
            }
        }

        // if let Some(account_id) = result.as_string() {
        //     set_account_state.set(account_id);
        // } else {
        //     set_account_state.set("hello.near".to_string());
        // }

      
    });

    Effect::new(move |_| {
        fetch_pool.dispatch(());
      });
    



    let UseClipboardReturn {
        is_supported,
        text,
        copied,
        copy,
    } = use_clipboard_with_options(UseClipboardOptions::default().read(true));   

    
    view! {
        <>
            {move || {
                let full_id = account.get();
                let shortened_id = if full_id.len() > 8 {
                    format!("{}...{}", &full_id[..8], &full_id[full_id.len() - 4..])
                } else {
                    full_id.clone()
                };
                if !shortened_id.is_empty() {
                    view! {
                        <>
                            <span>{shortened_id}</span>

                            <button on:click={
                                let copy = copy.clone();
                                move |_| copy(&full_id)
                            }>
                                <Show
                                    when={copied}
                                    fallback={|| {
                                        view! { <Icon icon={icondata::AiCopyOutlined} /> }
                                    }}
                                >
                                    Copied!
                                </Show>
                            </button>
                        </>
                    }
                        .into_any()
                } else {
                    view! {
                        <>
                            <div></div>
                        </>
                    }
                        .into_any()
                }
            }}
        </>
    }
}