use dioxus::prelude::*;

mod api;
mod components;
mod models;
mod pages;

fn main() {
    #[cfg(feature = "server")]
    {
        // Server-side: start Axum with Dioxus fullstack
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                let _ = tracing_subscriber::fmt()
                    .with_env_filter(
                        tracing_subscriber::EnvFilter::from_default_env()
                            .add_directive("signal_noise=debug".parse().unwrap()),
                    )
                    .try_init();

                let db = api::db::init_db().await.expect("Failed to init SurrealDB");
                api::db::apply_schema(&db).await.expect("Failed to apply schema");

                let state = api::AppState { db };

                dioxus_axum::launch_with_context(App, state, "0.0.0.0:8080").await;
            });
    }

    #[cfg(not(feature = "server"))]
    {
        // WASM client
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<pages::Route> {}
    }
}
