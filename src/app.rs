use std::env::var;

use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::{Client, config::Credentials};

#[derive(Debug, Clone)]
pub struct App {
    pub access_id_key: String,
    pub secret_access_key: String,
    pub api_address: String,
    pub storage_url: String,
    pub storage_region: String,
    pub storage_secret: String,
    pub client: Client,
}

impl App {
    pub async fn new() -> Self {
        _ = dotenvy::dotenv();
        let api_address = var("API_ADDRESS").unwrap();
        let storage_url = var("STORAGE_URL").unwrap();
        let storage_region = var("STORAGE_REGION").unwrap();
        let storage_secret = var("STORAGE_SECRET").unwrap();
        let access_id_key = var("ACCESS_ID_KEY").unwrap();
        let secret_access_key = var("STORAGE_SECRET").unwrap();
        // ----------------

        let credentials = Credentials::new(
            access_id_key.clone(),
            secret_access_key.clone(),
            None,
            None,
            "garage",
        );

        let config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(api_address.clone())
            .region(Region::new(storage_region.clone()))
            .credentials_provider(credentials)
            .load()
            .await;

        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true)
            .build();
        App {
            access_id_key,
            secret_access_key,
            api_address,
            storage_region,
            storage_secret,
            storage_url,
            client: Client::from_conf(s3_config),
        }
    }
}
