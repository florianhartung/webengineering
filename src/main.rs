#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::env;
    use std::sync::Arc;

    use axum::Router;
    use diesel::r2d2::{self, ConnectionManager};
    use diesel::PgConnection;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use webengineering::app::*;
    use webengineering::app_state::AppState;
    use webengineering::fileserv::file_and_error_handler;

    let _ = dotenvy::dotenv();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let database_pool = r2d2::Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    let state = AppState {
        leptos_options,
        database: Arc::new(database_pool),
    };
    let state_context = state.clone();

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(&state, routes, move || { provide_context(state_context.clone()); }, App)
        .fallback(file_and_error_handler)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
