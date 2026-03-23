use dioxus::prelude::*;

mod api;
mod components;
mod models;
mod pages;
mod server_fns;

fn main() {
    #[cfg(feature = "server")]
    {
        dioxus::server::serve(|| async {
            let _ = tracing_subscriber::fmt()
                .with_env_filter(
                    tracing_subscriber::EnvFilter::from_default_env()
                        .add_directive("signal_noise=debug".parse().unwrap()),
                )
                .try_init();

            let db = api::db::init_db().await.expect("Failed to init SurrealDB");
            api::db::apply_schema(&db).await.expect("Failed to apply schema");

            let state = api::AppState { db: db.clone() };

            // Expose the DB as an Axum extension so #[server] functions can extract it.
            let router = dioxus::server::router(App)
                .layer(axum::Extension(db))
                .nest("/api", api::routes::router(state));

            Ok(router)
        });
    }

    #[cfg(not(feature = "server"))]
    {
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<pages::Route> {}
    }
}
