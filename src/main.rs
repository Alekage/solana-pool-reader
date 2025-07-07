use axum::Router;
use axum::routing::get;
use handlers::fetch_handler::fetch_pools;

mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/api/pool-data/{token_mint_a}/{token_mint_b}",
        get(fetch_pools),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}