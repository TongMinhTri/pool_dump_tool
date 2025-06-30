use ::reqwest::blocking::Client;
use alloy_primitives::{U256, hex};
use alloy_sol_types::private::primitives::aliases::I24;
use serde_json::{Value, json};
use std::fs::File;
use std::io::{self, BufRead, Write};

fn read_pool_addresses(file_path: &str) -> Vec<String> {
    let mut addresses = Vec::new();
    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(address) = line {
                addresses.push(address);
            }
        }
    } else {
        eprintln!("Failed to open file: {}", file_path);
    }
    addresses
}

fn process_ticks(response_json: &Value) -> serde_json::Map<String, Value> {
    let mut ticks = serde_json::Map::new();
    if let Some(ticks_data) = response_json["result"]
        .get("ticks")
        .and_then(|t| t.as_object())
    {
        for (tick, tick_data) in ticks_data {
            let snake_case_tick_data = json!({
                "liquidity_gross": tick_data.get("liquidityGross").unwrap_or(&json!("0x0")),
                "liquidity_net": tick_data.get("liquidityNet").unwrap_or(&json!("0x0")),
                "fee_growth_outside_0x128": "0x0",
                "fee_growth_outside_1x128": "0x0"
            });
            ticks.insert(tick.clone(), snake_case_tick_data);
        }
    }
    ticks
}

fn decode_slot0(res_json: &str) -> Option<(U256, I24, &str)> {
    if let Ok(bytes) = hex::decode(res_json) {
        if bytes.len() >= 32 {
            let sqrt_price_x96 = U256::from_be_slice(&bytes[12..32]);
            let tick = I24::try_from_be_slice(&bytes[9..12]).unwrap();
            let fee_protocol = "0x0";
            return Some((sqrt_price_x96, tick, fee_protocol));
        }
    }
    None
}

fn process_v3_response(response_json: &Value, block_number_u64: u64) -> Value {
    let ticks = process_ticks(response_json);
    let res_json = response_json["result"]
        .get("slot0")
        .and_then(|s| s.as_str())
        .unwrap_or("");

    let slot0_data = if let Some((sqrt_price_x96, tick, fee_protocol)) = decode_slot0(res_json) {
        json!({
            "fee_protocol": fee_protocol.to_string(),
            "tick": tick.to_string(),
            "sqrt_price_x96": sqrt_price_x96.to_string()
        })
    } else {
        json!({})
    };

    json!({
        "state_block": block_number_u64,
        "pool": {
            "store": {
                "version": "v3",
                "protocol": "V3Pool",
                "fee": response_json["result"].get("fee").unwrap_or(&json!("0x0")),
                "tick_spacing": response_json["result"].get("tickSpacing").unwrap_or(&json!("0x0")),
                "liquidity": response_json["result"].get("liquidity").unwrap_or(&json!("0x0")),
                "tick_bitmap": response_json["result"].get("tickBitmap").unwrap_or(&json!({})),
                "ticks": ticks,
                "slot0": slot0_data,
                "protocol_fees": {
                    "token0": "0x0",
                    "token1": "0x0"
                },
                "fee_growth_global_0x128": "0x0",
                "fee_growth_global_1x128": "0x0"
            },
            "address": response_json["result"].get("address").unwrap_or(&json!("")),
            "token0": response_json["result"].get("token0").unwrap_or(&json!("")),
            "token1": response_json["result"].get("token1").unwrap_or(&json!("")),
            "dex": "Pancake",
            "protocol": "V3"
        }
    })
}

fn process_pool_addresses(
    client: &Client,
    url: &str,
    pool_addresses: Vec<String>,
    method: &str,
    options: Value,
    dump_prefix: &str,
) {
    for address in pool_addresses {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": [
                address,
                "latest",
                options
            ],
            "id": 1
        });

        match client.post(url).json(&payload).send() {
            Ok(resp) => {
                if resp.status().is_success() {
                    let output_file = format!("./snapshots/{}.{}.json", dump_prefix, address);
                    match resp.json::<Value>() {
                        Ok(response_json) => {
                            let block_number = U256::from_str_radix(
                                response_json["result"]["blockNumber"]
                                    .as_str()
                                    .unwrap_or("0x0")
                                    .trim_start_matches("0x"),
                                16,
                            )
                            .unwrap();
                            let block_number_u64 = u64::try_from(block_number).unwrap_or(0);

                            let formatted_response = if dump_prefix == "v2" {
                                json!({
                                    "state_block": block_number_u64,
                                    "pool": {
                                        "store": {
                                            "version": "v2",
                                            "protocol": "V2Pair",
                                            "reserve0": response_json["result"].get("reserve0").unwrap_or(&json!("0x0")),
                                            "reserve1": response_json["result"].get("reserve1").unwrap_or(&json!("0x0")),
                                            "fee": 2500
                                        },
                                        "address": response_json["result"].get("address").unwrap_or(&json!("")),
                                        "token0": response_json["result"].get("token0").unwrap_or(&json!("")),
                                        "token1": response_json["result"].get("token1").unwrap_or(&json!("")),
                                        "dex": "Pancake",
                                        "protocol": "V2"
                                    }
                                })
                            } else {
                                process_v3_response(&response_json, block_number_u64)
                            };

                            if let Err(e) = File::create(&output_file).and_then(|mut file| {
                                file.write_all(formatted_response.to_string().as_bytes())
                            }) {
                                eprintln!("Failed to write to file {}: {}", output_file, e);
                            } else {
                                println!("Response saved to {}", output_file);
                            }
                        }
                        Err(e) => eprintln!("Failed to parse JSON response: {}", e),
                    }
                } else {
                    eprintln!(
                        "Error: {} - {}",
                        resp.status(),
                        resp.text().unwrap_or_else(|_| "Unknown error".to_string())
                    );
                }
            }
            Err(e) => eprintln!("Request failed: {}", e),
        }
    }
}

fn main() {
    let url = "http://192.168.1.58:8575";
    let v2_file_path = "./panv2_pool_addresses.txt";
    let v3_file_path = "./panv3_pool_addresses.txt";

    let v2_pool_addresses = read_pool_addresses(v2_file_path);
    let v3_pool_addresses = read_pool_addresses(v3_file_path);

    let client = Client::new();

    let mut liquidity_slot = 5;
    let mut ticks_slot = 6;
    let mut tick_bitmap_slot = 7;
    let mut token0_offset = 2323;
    let mut token1_offset = 11276;
    let mut fee_offset = 11312;
    let mut tick_spacing_offset = 11240;

    process_pool_addresses(
        &client,
        url,
        v2_pool_addresses,
        "arb_getUniswapV2Pair",
        json!({}),
        "v2",
    );

    if v3_file_path == "./univ3_pool_addresses.txt" {
        liquidity_slot = 4;
        ticks_slot = 5;
        tick_bitmap_slot = 6;
        token0_offset = 2257;
        token1_offset = 10528;
        fee_offset = 10564;
        tick_spacing_offset = 10492;
    }

    process_pool_addresses(
        &client,
        url,
        v3_pool_addresses,
        "arb_getUniswapV3Pool",
        json!({
            "slot0Slot": 0,
            "liquiditySlot": liquidity_slot,
            "ticksSlot": ticks_slot,
            "tickBitmapSlot": tick_bitmap_slot,
            "token0Offset": token0_offset,
            "token1Offset": token1_offset,
            "feeOffset": fee_offset,
            "tickSpacingOffset": tick_spacing_offset
        }),
        "v3",
    );
}
