use tokio::net::TcpListener;

use crate::{app::App, types::ErrorResultType};

pub mod app;
pub mod emotion;
pub mod routes;
pub mod services;
pub mod types;

pub async fn run() -> ErrorResultType {
    let app = App::new().await;
    let router = routes::mount_routes(&app);
    let listener = TcpListener::bind(&app.api_address)
        .await
        .expect("Failed to initialize the TCP listener");
    println!("Server is running on {}", &app.api_address);
    axum::serve(listener, router.into_make_service()).await?;
    Ok(())
}
