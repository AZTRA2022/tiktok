#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tiktok::run().await?;
    Ok(())
}
