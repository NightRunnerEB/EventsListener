mod api;
mod processor;
mod input;

use reqwest::Client;
use chrono::Utc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let contract_address = input::get_contract_address();
    let client = Client::new();
    let mut current_timestamp = Utc::now().timestamp();
    let mut first_run = true;

    loop {
        match api::get_logs(&client, &contract_address, current_timestamp, &mut first_run).await {
            Ok(logs) => {
                current_timestamp = Utc::now().timestamp();
                for log in logs.iter().rev() {
                    processor::process_log(&log);
                }
            }
            Err(e) => eprintln!("Error occurred: {}", e),
        }

        sleep(Duration::from_secs(5)).await;
    }
}
