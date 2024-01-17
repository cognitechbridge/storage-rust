use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

pub async fn get_s3_client() -> Client {
    let region_provider = RegionProviderChain::default_provider();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);
    client
}