use axum::Extension;

use crate::app::App;

pub mod storage;
pub mod users;

pub fn mount_routes(app: &App) -> axum::Router {
    axum::Router::new()
        .nest("/users", users::mount_users_routes())
        .nest("/storage", storage::mount_storage_routes())
        .layer(Extension(app.clone()))
}
