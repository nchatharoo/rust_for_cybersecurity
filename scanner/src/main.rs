use rayon::prelude::*;
mod subdomains;
mod ports;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let http_timeout= Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;
    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();

    let subdomains = subdomains::enumerate(&http_client, target).await?;

}