
# Events Listener for MultiversX Smart Contract

This is a simple Rust-based listener application designed to fetch and display logs from a specific smart contract deployed on the MultiversX blockchain. The application uses the MultiversX API to retrieve transaction data and extract log events.

## Features

- Fetches transaction logs from a specified smart contract
- Filters and displays relevant information such as function identifier, timestamp, and operation messages
- Supports both real-time monitoring and historical log fetching
- Automatically updates logs every 5 seconds

## Requirements

To run the listener, you will need:

- Rust (stable toolchain)
- Cargo (Rust package manager)
- Internet connection to interact with the MultiversX blockchain

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/NightRunnerEB/EventsListener.git
   cd EventsListener
   ```

2. Build the project:

   ```bash
   cargo build
   ```

3. Run the project:

   ```bash
   cargo run
   ```

## Usage

1. When you run the application, it will ask for a smart contract address. Enter the smart contract address you want to monitor.

   Example:
   
   ```
   Please enter the contract address:
   erd1qqqqqqqqqqqqqpgq44x7y9vpgj8xw9zlgz448waa30v8fadx0cpsvz84t9
   ```

2. The application will then begin fetching logs from that smart contract every 5 seconds and display the following details:
   - Action
   - Timestamp
   - Id(txHash)
   - Type
   - Data
   - Message

3. Press `Ctrl + C` to stop the application at any time.

## Example Output

```
Action: claim
Timestamp: 2024-10-09 07:26:20 UTC
Id: 2733af3472b5dfc09660171c48987cc380ad6974415fc8799e34f47e718fbbbc
Type: signalError
Data: user error
Message: cannot claim before deadline
```