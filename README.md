# pool_dump_tool

## Overview
This project is designed to dump pool data to the latest block of a blockchain node.

## Usage

1. **Dump Pools**
   - Run the `main.rs` file to dump the pools listed in the `.txt` files (e.g., `panv2_pool_addresses.txt`, `panv3_pool_addresses.txt`, `univ3_pool_addresses.txt`) to the latest block of the full node.

     ```bash
     cargo run --bin pool_dump_new
     ```

2. **Update Configuration**
   - After dumping the pools, update the `to_block` value in the `product.toml` configuration file to the desired block number.

3. **Sync pools to the desired block**
   - Execute the `market-dump` binary to sync the pools in ./snapshots to the desired block.

     ```bash
     APP_ENV=product ./market-dump
     ```

## Configuration

The configuration file `product.toml` contains the following key settings:

- `app_env`: The environment setting (e.g., `product`).
- `folder_snapshot`: Directory for snapshots.
- `folder_dump`: Directory for dumps.
- `block_step`: Number of blocks to process in each step.
- `to_block`: The latest block number to process.
- `rpc.http`: The RPC endpoint of the blockchain node.
- `bsc_scan.api_key`: API key for BSC scan.

## Prerequisites

- Rust installed on your system.
- Access to a blockchain node RPC endpoint.

## Notes

- Ensure the `.txt` files containing pool addresses are correctly formatted and placed in the root directory.
- The `to_block` value in `product.toml` must be updated manually after running `main.rs` to reflect the desired block number.
