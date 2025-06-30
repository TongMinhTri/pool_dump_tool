# Arbitrage APIs Guide

This guide describes how to use the new Ethereum node RPC APIs for retrieving Uniswap pool information for arbitrage purposes.

## Prerequisites

To use these APIs, your Ethereum node must have the `arb` HTTP API enabled. This can be done in two ways:

### Command Line
```bash
geth --http --http.api "...,arb" --http.port 8545
```

### Configuration File
Add the following to your node's configuration file:
```toml
[Node]
HTTPModules = [..., "arb"]
```

## API Methods

### 1. arb_getUniswapV2Pair

Retrieves Uniswap V2 pair information for a given address at a specific block.

#### Parameters

1. `address` (string) - The contract address of the Uniswap V2 pair
2. `blockNumberOrHash` (string) - Block number (hex), block hash, or block tag ("latest", "earliest", "pending")
3. `options` (object, optional) - Configuration for storage slots:
   - `token0Slot` (string, optional) - Storage slot for token0 address (default: 6)
   - `token1Slot` (string, optional) - Storage slot for token1 address (default: 7) 
   - `reservesSlot` (string, optional) - Storage slot for reserves data (default: 8)

**Note:** The `options` parameter can be omitted when querying original Uniswap V2 contracts, as the default slot values are correct. However, for modified versions like PancakeSwap or other forks, you must provide the correct storage slot values or the results will be incorrect. See [Appendix: Fork-Specific Options](#appendix-fork-specific-options) for pre-analyzed configurations.

#### Response

Returns a UniswapV2Pair object containing:
- `address` - The pair contract address
- `token0` - Address of the first token
- `token1` - Address of the second token
- `reserve0` - Reserve amount of token0
- `reserve1` - Reserve amount of token1
- `blockTimestampLast` - Timestamp of the last block when reserves were updated
- `blockNumber` - Block number of the data
- `blockHash` - Block hash of the data

#### Example Usage

```bash
# Basic call with latest block
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc": "2.0",
    "method": "arb_getUniswapV2Pair",
    "params": [
      "0x0e64464B54b5fb85487Ea5fCc9C73aD86952dC7e",
      "latest",
      {}
    ],
    "id": 1
  }' \
  http://localhost:8545

# Call with specific block number and custom slots
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc": "2.0",
    "method": "arb_getUniswapV2Pair",
    "params": [
      "0xa478c2975ab1ea89e8196811f51a7b7ade33eb11",
      "0x12a05f200",
      {
        "token0Slot": 6,
        "token1Slot": 7,
        "reservesSlot": 8
      }
    ],
    "id": 1
  }' \
  http://localhost:8545
```

#### Example Response

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "address": "0xa478c2975ab1ea89e8196811f51a7b7ade33eb11",
    "token0": "0xa0b86a33e6411c4bb72b5b67ba2ce6b6d9c8f5d2",
    "token1": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    "reserve0": "0x1bc16d674ec80000",
    "reserve1": "0x6c6b935b8bbd400000",
    "blockTimestampLast": "0x635a1234",
    "blockNumber": "0x12a05f200",
    "blockHash": "0x1234567890abcdef..."
  }
}
```

### 2. arb_getUniswapV3Pool

Retrieves Uniswap V3 pool information for a given address at a specific block.

#### Parameters

1. `address` (string) - The contract address of the Uniswap V3 pool
2. `blockNumberOrHash` (string) - Block number (hex), block hash, or block tag ("latest", "earliest", "pending")
3. `options` (object, optional) - Configuration for storage slots and code offsets:
   - `slot0Slot` (string, optional) - Storage slot for slot0 data (default: "0)
   - `liquiditySlot` (string, optional) - Storage slot for liquidity (default: 4)
   - `ticksSlot` (string, optional) - Storage slot for ticks mapping (default: 5)
   - `tickBitmapSlot` (string, optional) - Storage slot for tick bitmap (default: 6)
   - `token0Offset` (number, optional) - Bytecode offset for token0 address (default: 2257)
   - `token1Offset` (number, optional) - Bytecode offset for token1 address (default: 10528)
   - `feeOffset` (number, optional) - Bytecode offset for fee (default: 10564)
   - `tickSpacingOffset` (number, optional) - Bytecode offset for tick spacing (default: 10492)

**Note:** The `options` parameter can be omitted when querying original Uniswap V3 contracts, as the default slot and offset values are correct. However, for modified versions like PancakeSwap V3 or other forks, you must provide the correct storage slot values and bytecode offsets or the results will be incorrect. See [Appendix: Fork-Specific Options](#appendix-fork-specific-options) for pre-analyzed configurations.

#### Response

Returns a UniswapV3Pool object containing:
- `address` - The pool contract address
- `token0` - Address of the first token
- `token1` - Address of the second token
- `fee` - Pool fee tier
- `tickSpacing` - Tick spacing for the pool
- `slot0` - Current pool state (price, tick, etc.)
- `liquidity` - Current pool liquidity
- `tickBitmap` - Mapping of initialized tick bitmap words
- `ticks` - Mapping of initialized tick information with `liquidityNet` and `liquidityGross`
- `blockNumber` - Block number of the data
- `blockHash` - Block hash of the data

#### Example Usage

```bash
# Basic call with latest block
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc": "2.0",
    "method": "arb_getUniswapV3Pool",
    "params": [
      "0xFc3cfbe887009d4B9392801899bc89eB319E8E79",
      "latest",
      {}
    ],
    "id": 1
  }' \
  http://localhost:8545

# Call with specific block and custom configuration
curl -X POST -H "Content-Type: application/json" \
  --data '{
    "jsonrpc": "2.0",
    "method": "arb_getUniswapV3Pool",
    "params": [
      "0xD0e226f674bBf064f54aB47F42473fF80DB98CBA",
      "0x12a05f200",
      {
        "slot0Slot": 0,
        "liquiditySlot": 4,
        "ticksSlot": 5,
        "tickBitmapSlot": 6,
        "token0Offset": 2257,
        "token1Offset": 10528,
        "feeOffset": 10564,
        "tickSpacingOffset": 10492
      }
    ],
    "id": 1
  }' \
  http://localhost:8545
```

#### Example Response

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "address": "0xFc3cfbe887009d4B9392801899bc89eB319E8E79",
    "token0": "0xa0b86a33e6411c4bb72b5b67ba2ce6b6d9c8f5d2",
    "token1": "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    "fee": "0x1388",
    "tickSpacing": "0x3c",
    "slot0": "0x1234567890abcdef...",
    "liquidity": "0x1bc16d674ec80000",
    "tickBitmap": {
      "-1": "0x8000000000000000000000000000000000000000000000000000000000000000",
      "0": "0x4000000000000000000000000000000000000000000000000000000000000001"
    },
    "ticks": {
      "-887220": {
        "liquidityNet": "0x1bc16d674ec80000",
        "liquidityGross": "0x1bc16d674ec80000"
      },
      "887220": {
        "liquidityNet": "-0x1bc16d674ec80000",
        "liquidityGross": "0x1bc16d674ec80000"
      }
    },
    "blockNumber": "0x12a05f200",
    "blockHash": "0x1234567890abcdef..."
  }
}
```

## Error Handling

Both APIs will return standard JSON-RPC error responses for invalid parameters or node errors:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32602,
    "message": "Invalid params"
  }
}
```

## Use Cases

These APIs are particularly useful for:
- **Arbitrage bots** - Quickly retrieve pool states across different blocks
- **MEV strategies** - Analyze pool conditions before transaction execution  
- **Analytics** - Historical analysis of pool states and liquidity
- **Price discovery** - Real-time pool monitoring for trading opportunities

## Notes

- The V3 API automatically searches for and returns only initialized ticks to optimize response size
- Custom slot and offset parameters allow compatibility with pool variants or forks
- Block data is included in responses for precise arbitrage timing
- All numeric values are returned as hex strings following Ethereum JSON-RPC conventions
- For popular forks, see the [Appendix: Fork-Specific Options](#appendix-fork-specific-options) for pre-analyzed configuration values

## Appendix: Fork-Specific Options

This section contains pre-analyzed configuration options for popular Uniswap forks. These values have been determined by analyzing the contract bytecode and storage layouts of each fork.

### PancakeSwap V2

PancakeSwap V2 uses identical storage slot assignments to the original Uniswap V2 contracts. No custom options are required - you can omit the `options` parameter entirely when querying PancakeSwap V2 pairs.

### PancakeSwap V3

PancakeSwap V3 uses different storage slots and bytecode offsets compared to the original Uniswap V3. Use these options when querying PancakeSwap V3 pools:

```json
{
  "liquiditySlot": 5,
  "ticksSlot": 6, 
  "tickBitmapSlot": 7,
  "token0Offset": 2323,
  "token1Offset": 11276,
  "feeOffset": 11312,
  "tickSpacingOffset": 11240
}
```

### Future Fork Support

Additional fork configurations will be added here as they are analyzed. If you need configuration for a specific fork not listed here, you will need to:

1. Analyze the contract bytecode to determine the correct bytecode offsets for immutable values
2. Review the contract source code to identify any changes in storage slot assignments
3. Test the configuration with known pool addresses to verify correctness

**Note:** Always verify configurations with known pool data before using in production, as contract implementations may vary even within the same fork family.