// use crate::components::counter_btn::Button;
use crate::components::navigation::nav::Nav;
use leptos::prelude::*;
use crate::components::posts::create_post::CreatePost;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>
            <Nav />
            <main class="min-h-screen bg-white dark:bg-gray-900 dark:text-white text-gray-900 p-4">
                <section class="text-center py-10">
                    <h1 class="text-4xl font-bold text-purple-600 dark:text-purple-400">
                        Smart Trade
                    </h1>
                    <p class="mt-4 text-lg text-gray-700 dark:text-gray-300">
                        "Trading platform that utilizes AI agents to automate your trades"
                    </p>
                    <p class="mt-2 text-gray-600 dark:text-gray-400">
                        "Smart Trade is a trading system that leverages the power of artificial intelligence (AI) to analyze your trade history, identify profitable trades, and execute them automatically."
                    </p>
                </section>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 px-4">
                    <div class="bg-purple-300 dark:bg-purple-700 p-6 rounded-2xl shadow-lg dark:hover:shadow-purple-500 hover:shadow-purple-300 transition-shadow">
                        <h2 class="text-2xl font-semibold">"Analyze Trade History"</h2>
                        <p class="mt-2 text-gray-800 dark:text-gray-200">
                            "Use your past trade data to inform future trades."
                        </p>
                    </div>

                    <div class="bg-blue-300 dark:bg-blue-700 p-6 rounded-2xl shadow-lg dark:hover:shadow-blue-500 hover:shadow-blue-300 transition-shadow">
                        <h2 class="text-2xl font-semibold">"Identify Profitable Trades"</h2>
                        <p class="mt-2 text-gray-800 dark:text-gray-200">
                            "Make use of AI to identify profitable trades"
                        </p>
                    </div>

                    <div class="bg-green-300 dark:bg-green-700 p-6 rounded-2xl shadow-lg dark:hover:shadow-green-500 hover:shadow-green-300 transition-shadow">
                        <h2 class="text-2xl font-semibold">"AI Execution"</h2>
                        <p class="mt-2 text-gray-800 dark:text-gray-200">
                            "Leverage AI to execute trades automatically."
                        </p>

                    </div>
                </div>

                <CreatePost />

            </main>
        </ErrorBoundary>
    }
}
