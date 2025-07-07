use axum::Router;
use axum::routing::get;
use handlers::fetch_handler::fetch_pools;

mod handlers;
mod config;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env();
    let addr = format!("{}:{}", config.server.host, config.server.port);

    println!("Starting Solana Pool Reader server on {}", addr);

    let app = Router::new().route(
        "/api/pool-data/{token_mint_a}/{token_mint_b}",
        get(fetch_pools),
    );

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server listening on {}", addr);

    axum::serve(listener, app).await?;
    Ok(())
}