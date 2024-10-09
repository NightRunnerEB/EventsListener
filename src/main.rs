use reqwest::Client;
use std::str;
use anyhow::anyhow;
use std::io::{self, Write};
use chrono::{DateTime, Utc};
use serde_json::Value;
use tokio::time::{sleep, Duration};

const API_URL: &str = "https://devnet-api.multiversx.com";

#[tokio::main]
async fn main() {

    let contract_address = get_contract_address();

    let client = Client::new();
    let mut current_timestamp = Utc::now().timestamp();
    let mut first_run = true;

    loop {
        match get_logs(&client, &contract_address, current_timestamp, &mut first_run).await {
            Ok(logs) => {
                current_timestamp = Utc::now().timestamp();
                for log in logs.iter().rev() {
                    let action = log["function"].as_str().unwrap_or("Unknown Identifier");
                    let timestamp = log["timestamp"].as_u64().unwrap_or(0);
                    let naive_datetime = DateTime::<Utc>::from_timestamp(timestamp as i64, 0).unwrap();

                    println!("\nAction: {action}\nTimestamp: {naive_datetime}");

                    process_results(&log);
                }
            }
            Err(e) => eprintln!("Error occurred: {}", e)
        }

        sleep(Duration::from_secs(5)).await;
    }
}


async fn get_logs(client: &Client, address: &str, timestamp: i64, first_run: &mut bool) -> Result<Vec<Value>, anyhow::Error> {
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

async fn get_transaction_details(client: &Client, tx_hash: &str) -> Result<Value, anyhow::Error> {
    let url = format!("{API_URL}/transactions/{tx_hash}");
    let response = client.get(&url).send().await?;
    let body = response.json::<Value>().await?;
    Ok(body)
}

fn process_results(results: &Value) {
    if let Some(operations) = results["operations"].as_array() {
        for operation in operations {
            let id = operation["id"].as_str().unwrap_or("No identifier");
            let action = operation["action"].as_str().unwrap_or("No identifier");
            let operation_data = operation["data"].as_str().unwrap_or("No Data");
            let operation_message = operation["message"].as_str().unwrap_or("No Message");

            println!("Id: {}", id);
            println!("Type: {}", action);
            match parse_hex_data(operation_data) {
                Ok(result) => println!("Data: {}", result),
                Err(e) => eprintln!("Failed to decode: {}", e),
            }
            println!("Message: {}", operation_message);
        }
    } else {
        println!("No operations found for this transaction.");
    }
}

fn get_contract_address() -> String {
    print!("Please enter the contract address: ");
    io::stdout().flush().unwrap();
    let mut contract_address = String::new();
    io::stdin().read_line(&mut contract_address).expect("Failed to read line");
    contract_address.trim().to_string()
}

fn parse_hex_data(hex_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let hex_str = hex_data.trim_start_matches('@');
    let bytes = hex::decode(hex_str)?;
    let decoded_str = String::from_utf8(bytes)?;
    Ok(decoded_str)
}
