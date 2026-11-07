use axum::Extension;

use crate::app::App;

pub async fn list_buckets(Extension(app): Extension<App>) -> &'static str {
    let buckets = app.client.list_buckets().send().await.unwrap();
    if let Some(bucks) = buckets.buckets {
        println!(
            "bucket: {:?}",
            bucks
                .iter()
                .map(|b| b.name.as_ref().unwrap().contains("videp"))
                .collect::<Vec<_>>()
        );
    }
    return "Buckets printed well to dbg output";
}
