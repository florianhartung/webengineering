use std::sync::Arc;

use axum::extract::FromRef;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel::r2d2;
use leptos::LeptosOptions;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub database: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
}
