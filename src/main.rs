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

                use dioxus_fullstack::prelude::{DioxusRouterExt, ServeConfigBuilder};
                use std::sync::Arc;

                let state_for_ssr = state.clone();
                let context_providers = Arc::new(vec![
                    Box::new(move || -> Box<dyn std::any::Any> { Box::new(state_for_ssr.clone()) })
                        as Box<dyn Fn() -> Box<dyn std::any::Any> + Send + Sync + 'static>,
                ]);

                let cfg = ServeConfigBuilder::default().context_providers(context_providers);

                let router = axum::Router::new()
                    .nest("/api", api::routes::router(state))
                    .serve_dioxus_application(cfg, App);

                let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
                    .await
                    .expect("Failed to bind port 8080");
                tracing::info!("Listening on 0.0.0.0:8080");
                axum::serve(listener, router.into_make_service())
                    .await
                    .expect("Server error");
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
