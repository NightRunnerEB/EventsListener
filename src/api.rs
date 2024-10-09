use reqwest::Client;
use anyhow::{Result, anyhow};
use serde_json::Value;

const API_URL: &str = "https://devnet-api.multiversx.com";

pub async fn get_logs(client: &Client, address: &str, timestamp: i64, first_run: &mut bool) -> Result<Vec<Value>> {
    let url = if *first_run {
        *first_run = false;
        format!("{API_URL}/accounts/{address}/transactions?before={timestamp}")
    } else {
        format!("{API_URL}/accounts/{address}/transactions?after={timestamp}")
    };

    let response = client.get(&url).send().await?;
    let body = response.json::<Value>().await?;

    if let Some(logs) = body.as_array() {
        let mut detailed_logs = vec![];
        for log in logs {
            let tx_hash = log["txHash"].as_str().unwrap_or_default();
            let details = get_transaction_details(client, tx_hash).await?;
            detailed_logs.push(details);
        }
        Ok(detailed_logs)
    } else {
        Err(anyhow!("Events not found"))
    }
}

async fn get_transaction_details(client: &Client, tx_hash: &str) -> Result<Value> {
    let url = format!("{API_URL}/transactions/{tx_hash}");
    let response = client.get(&url).send().await?;
    let body = response.json::<Value>().await?;
    Ok(body)
}
