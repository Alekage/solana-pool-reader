use axum::Json;
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenMints {
    token_mint_a: String,
    token_mint_b: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    pool_id: String,
    tvl: f64,
    price: f64,
}

///////////////////// RAYDIUM /////////////////////
pub async fn fetch_pool_data_raydium(token_mint_a: String, token_mint_b: String) -> Vec<Pool> {
    let url = format!(
        "https://api-v3.raydium.io/pools/info/mint?mint1={}&mint2={}&poolType=all&poolSortField=default&sortType=desc&pageSize=1000&page=1",
        token_mint_a, token_mint_b
    );

    let response = reqwest::get(&url).await.expect("Failed to fetch data");

    let result = response
        .json::<Value>()
        .await
        .expect("Failed to parse JSON");

    let mut pools_returned: Vec<Pool> = vec![];

    if let Some(pools) = result["data"]["data"].as_array() {
        for pool in pools {
            let pool_id = pool["id"].as_str().unwrap_or("Unknown").to_string();
            let tvl = pool["tvl"].as_f64().unwrap_or(0.0);
            let price = pool["price"].as_f64().unwrap_or(0.0);

            let pool = Pool {
                pool_id,
                tvl,
                price,
            };

            pools_returned.push(pool);
        }
    } else {
        println!(
            "No data found for mints: {}, {}",
            token_mint_a, token_mint_b
        );
    }
    pools_returned
}

///////////////////// ORCA /////////////////////
pub async fn fetch_pool_data_orca(token_mint_a: String, token_mint_b: String) -> Vec<Pool> {
    let url = format!(
        "https://api.orca.so/v2/solana/pools?token={}&limit=65535",
        token_mint_a
    );

    let response = reqwest::get(&url)
        .await
        .expect("Failed to fetch from Orca API");

    let result = response
        .json::<Value>()
        .await
        .expect("Failed to deserialize Orca response");

    let mut pools_returned: Vec<Pool> = vec![];

    let pools = result
        .get("data")
        .and_then(|data| data.as_array())
        .map(|pools| {
            pools
                .iter()
                .filter(|pool| {
                    let token_a = pool
                        .get("tokenA")
                        .and_then(|t| t.get("address"))
                        .and_then(|a| a.as_str());
                    let token_b = pool
                        .get("tokenB")
                        .and_then(|t| t.get("address"))
                        .and_then(|b| b.as_str());

                    (token_a == Some(&token_mint_a) && token_b == Some(&token_mint_b))
                        || (token_b == Some(&token_mint_b) && token_a == Some(&token_mint_a))
                })
                .cloned()
                .collect::<Vec<Value>>()
        })
        .unwrap_or_else(Vec::new);

    for pool in pools {
        let tvl = pool["tvlUsdc"]
            .as_str()
            .unwrap_or("0.0")
            .parse::<f64>()
            .unwrap_or(0.0);

        let price = pool["price"]
            .as_str()
            .unwrap_or("0.0")
            .parse::<f64>()
            .unwrap_or(0.0);

        let pool_id = pool["address"].as_str().unwrap_or("Unknown").to_string();

        let orca_pool = Pool {
            pool_id,
            tvl,
            price,
        };

        pools_returned.push(orca_pool);
    }
    pools_returned
}

///////////////////// METEORA /////////////////////
pub async fn fetch_pool_data_meteora(token_mint_a: String, token_mint_b: String) -> Vec<Pool> {
    let url = format!(
        "https://dlmm-api.meteora.ag/pair/all_by_groups?include_pool_token_pairs={}-{}",
        token_mint_a, token_mint_b
    );

    let response = reqwest::get(&url)
        .await
        .expect("Failed to fetch from Meteora API");

    let result = response
        .json::<Value>()
        .await
        .expect("Failed to deserialize Meteora response");

    let mut pools_returned: Vec<Pool> = vec![];

    if let Some(groups) = result["groups"].as_array() {
        for group in groups {
            if let Some(pairs) = group["pairs"].as_array() {
                for pair in pairs {
                    let tvl = pair["liquidity"]
                        .as_str()
                        .unwrap_or("0.0")
                        .parse::<f64>()
                        .unwrap_or(0.0);

                    let price = pair["current_price"].as_f64().unwrap_or(0.0);

                    let pool_id = pair["address"].as_str().unwrap_or("Unknown").to_string();

                    let pool = Pool {
                        pool_id,
                        tvl,
                        price,
                    };

                    pools_returned.push(pool);
                }
            }
        }
    } else {
        println!(
            "No data found for mints: {}, {}",
            token_mint_a, token_mint_b
        );
    }

    pools_returned
}

fn find_best_pool_by_tvl<'a>(pools: Vec<Pool>) -> Option<Pool> {
    println!("Number of pools: {:?}", pools.len());

    pools.into_iter().max_by(|a, b| {
        a.tvl
            .partial_cmp(&b.tvl)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

// pub async fn fetch_pools(
//     Path(TokenMints {
//         token_mint_a,
//         token_mint_b,
//     }): Path<TokenMints>,
// ) -> impl axum::response::IntoResponse {
//     let raydium_handle = tokio::spawn(fetch_pool_data_raydium(
//         token_mint_a.clone(),
//         token_mint_b.clone(),
//     ));

//     let orca_handle = tokio::spawn(fetch_pool_data_orca(
//         token_mint_a.clone(),
//         token_mint_b.clone(),
//     ));

//     let meteora_handle = tokio::spawn(fetch_pool_data_meteora(
//         token_mint_a.clone(),
//         token_mint_b.clone(),
//     ));

//     // let (raydium, orca, meteora) = tokio::join!(
//     //     fetch_pool_data_raydium(token_mint_a.clone(), token_mint_b.clone()),
//     //     fetch_pool_data_orca(token_mint_a.clone(), token_mint_b.clone()),
//     //     fetch_pool_data_meteora(token_mint_a.clone(), token_mint_b.clone())
//     // );

//     let (raydium, orca, meteora) = tokio::join!(raydium_handle, orca_handle, meteora_handle);

//     let mut all_pools = vec![];

//     all_pools.extend(raydium.expect("Raydium task panicked"));
//     all_pools.extend(orca.expect("Orca task panicked"));
//     all_pools.extend(meteora.expect("Meteora task panicked"));

//     let best_pool_task = tokio::task::spawn_blocking(|| find_best_pool_by_tvl(all_pools));
//     let best_pool = best_pool_task.await.unwrap();

//     if let Some(best_pool) = best_pool {
//         Json(json!(best_pool))
//     } else {
//         Json(json!({ "error": "No pools found" }))
//     }
// }

pub async fn fetch_pools(
    Path(TokenMints {
        token_mint_a,
        token_mint_b,
    }): Path<TokenMints>,
) -> impl axum::response::IntoResponse {
    let raydium = fetch_pool_data_raydium(token_mint_a.clone(), token_mint_b.clone()).await;
    let orca = fetch_pool_data_orca(token_mint_a.clone(), token_mint_b.clone()).await;
    let meteora = fetch_pool_data_meteora(token_mint_a.clone(), token_mint_b.clone()).await;

    let mut all_pools = vec![];

    all_pools.extend(raydium);
    all_pools.extend(orca);
    all_pools.extend(meteora);

    let best_pool_task = tokio::task::spawn_blocking(|| find_best_pool_by_tvl(all_pools));
    let best_pool = best_pool_task.await.unwrap();

    if let Some(best_pool) = best_pool {
        Json(json!(best_pool))
    } else {
        Json(json!({ "error": "No pools found" }))
    }
}

// pub async fn fetch_pools(
//     Path(TokenMints {
//         token_mint_a,
//         token_mint_b,
//     }): Path<TokenMints>,
// ) -> impl axum::response::IntoResponse {
//     let mut set = tokio::task::JoinSet::new();

//     set.spawn(fetch_pool_data_raydium(
//         token_mint_a.clone(),
//         token_mint_b.clone(),
//     ));
//     set.spawn(fetch_pool_data_orca(
//         token_mint_a.clone(),
//         token_mint_b.clone(),
//     ));
//     set.spawn(fetch_pool_data_meteora(
//         token_mint_a.clone(),
//         token_mint_b.clone(),
//     ));

//     let mut all_pools = Vec::new();

//     while let Some(result) = set.join_next().await {
//         match result {
//             Ok(pools) => all_pools.extend(pools),
//             Err(e) => {
//                 println!("Error fetching data {}", e);
//             }
//         }
//     }

//     if all_pools.is_empty() {
//         return Json(json!({ "error": "No pools found" }));
//     }

//     match tokio::task::spawn_blocking(move || find_best_pool_by_tvl(all_pools)).await {
//         Ok(Some(best_pool)) => Json(json!(best_pool)),
//         Ok(None) => Json(json!({ "error": "No suitable pool found" })),
//         Err(e) => {
//             println!("Error finding best pool: {}", e);
//             Json(json!({ "error": "Failed to process pool data" }))
//         }
//     }
// }
