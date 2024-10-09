use serde_json::Value;
use chrono::DateTime;
use bech32::{encode, Variant, ToBase32};
use std::str;

pub fn process_log(log: &Value) {
    let action = log["function"].as_str().unwrap_or("Unknown Identifier");
    let timestamp = log["timestamp"].as_u64().unwrap_or(0);
    let naive_datetime = DateTime::from_timestamp(timestamp as i64, 0).unwrap();
    println!("\nAction: {action}\nTimestamp: {naive_datetime}");

    if let Some(events) = log["logs"]["events"].as_array() {
        process_first_event(events);
    } else {
        println!("No logs found for this transaction.");
    }
}

pub fn process_first_event(logs: &Vec<Value>) {
    if let Some(first_event) = logs.first() {
        let address = first_event["address"].as_str().unwrap_or("Unknown address");
        let order = first_event["order"].as_i64().unwrap_or(0);
        println!("Address: {address}");
        println!("Order: {order}");
        
        let empty_array = vec![];
        let topics = first_event["topics"].as_array().unwrap_or(&empty_array);
        for (i, topic) in topics.iter().enumerate() {
            match decode_base64(topic) {
                Ok(decoded) => println!("Topic {}: {}", i + 1, decoded),
                Err(err) => println!("Error: {}", err),
            }
        }
    } else {
        println!("No events found.");
    }
}

pub fn decode_base64(value: &Value) -> Result<String, String> {
    if let Some(encoded_str) = value.as_str() {
        match base64::decode(encoded_str) {
            Ok(decoded_bytes) => match decoded_bytes.len() {
                32 => encode("erd", decoded_bytes.to_base32(), Variant::Bech32)
                    .map_err(|_| "Failed to encode to Bech32".to_string()),
                2 => {
                    let number = u16::from_be_bytes([decoded_bytes[0], decoded_bytes[1]]);
                    Ok(number.to_string())
                }
                _ => str::from_utf8(&decoded_bytes)
                    .map(|s| s.to_string())
                    .map_err(|_| "Failed to decode UTF-8 string".to_string()),
            },
            Err(_) => Err("Failed to decode Base64".to_string()),
        }
    } else {
        Err("Provided value is not a valid string".to_string())
    }
}
