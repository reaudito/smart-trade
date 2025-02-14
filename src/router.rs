use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use crate::pages::home::Home;
use crate::components::trade_posts::trade_history::MakeTrade;
use crate::components::trade_posts::get_pools_details::GetPoolsDetails;
use crate::components::trade_posts::get_pools_paginate::GetPoolsPaginate;
use crate::components::trade_posts::get_pool_details::GetPoolDetails;
#[component]
pub fn RouterApp() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| "Not Found.">
                <Route path=path!("/") view=Home />
                <Route path=path!("/trade") view=MakeTrade />
                <Route path=path!("/get-pools") view=GetPoolsDetails />
                <Route path=path!("/get-pools-paginate") view=GetPoolsPaginate />
                <Route path=path!("/get-pool/:pool_id") view=GetPoolDetails />

            </Routes>
        </Router>
    }
        }