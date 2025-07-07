# Solana Pool Reader

A Rust-based web service that aggregates pool data from multiple Solana DEX platforms (Raydium, Orca, and Meteora) to find the best liquidity pools for token pairs.

## Features

- **Multi-DEX Aggregation**: Fetches pool data from Raydium, Orca, and Meteora APIs
- **Concurrent Processing**: Uses async/await with tokio for parallel API requests
- **Best Pool Selection**: Automatically finds the pool with the highest TVL
- **Error Resilience**: Graceful handling of API failures and timeouts
- **Configurable**: Environment-based configuration

## API Endpoint

```
GET /api/pool-data/{token_mint_a}/{token_mint_b}
```

### Parameters
- `token_mint_a`: First token's mint address
- `token_mint_b`: Second token's mint address

### Response Format

**Success:**
```json
{
  "pool_id": "pool_address",
  "tvl": 1234567.89,
  "price": 1.23
}
```

**Error:**
```json
{
  "error": "No pools found",
  "tokens": {
    "token_mint_a": "token_a_address",
    "token_mint_b": "token_b_address"
  }
}
```

## Configuration

Environment variables:

- `HOST`: Server host (default: "0.0.0.0")
- `PORT`: Server port (default: 3000)
- `API_TIMEOUT_SECS`: API request timeout in seconds (default: 10)

## Running the Application

1. **Install dependencies:**
   ```bash
   cargo build
   ```

2. **Run the server:**
   ```bash
   cargo run
   ```

3. **With custom configuration:**
   ```bash
   HOST=127.0.0.1 PORT=8080 cargo run
   ```

## Architecture Improvements

### 1. Error Handling
- Replaced `unwrap()` and `expect()` with proper `Result` types
- Custom error types using `thiserror`
- Graceful handling of API failures

### 2. Concurrency
- Parallel API requests using `tokio::spawn` and `tokio::join!`
- Non-blocking pool processing
- Improved response times

### 3. Configuration Management
- Centralized configuration with environment variable support
- Type-safe configuration structs
- Default values with override capability

### 4. Code Organization
- Modular structure with separate handler and config modules
- Clear separation of concerns
- Improved maintainability

### 5. HTTP Client Improvements
- Request timeouts to prevent hanging requests
- Status code validation
- Better error messages

## Dependencies

- `axum`: Web framework
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `serde`: Serialization/deserialization
- `thiserror`: Error handling

## Development

The project follows Rust best practices:
- Proper error handling with `Result` types
- Async/await for non-blocking operations
- Type safety throughout the codebase
- Clear module structure
- Comprehensive error messages 