use leptos::prelude::*;
use leptos_icons::*;
use leptos_use::{
    use_clipboard_with_options, use_permission, UseClipboardOptions, UseClipboardReturn,
};

use codee::string::JsonSerdeCodec;



#[component]
pub fn AccountNav() -> impl IntoView {

    let (account_state, set_account_state) = signal("hello.near".to_string());

    let UseClipboardReturn {
        is_supported,
        text,
        copied,
        copy,
    } = use_clipboard_with_options(UseClipboardOptions::default().read(true));   

    
     view!{

        <>
        {move || {
            let full_id = account_state();
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