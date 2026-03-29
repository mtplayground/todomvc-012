#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::sqlite::SqlitePoolOptions;
    use todomvc::app::App;

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./todos.db?mode=rwc".to_string());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let addr = leptos_options.site_addr;
    let site_root = &leptos_options.site_root;
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback_service(tower_http::services::ServeDir::new(site_root))
        .layer(axum::Extension(pool))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect(&format!("Failed to bind to {addr}"));

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
