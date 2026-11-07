use crate::services::storage::list_buckets;
use axum::routing::get;

pub fn mount_storage_routes() -> axum::Router {
    let app = axum::Router::new().route("/list", get(list_buckets));
    app
}
