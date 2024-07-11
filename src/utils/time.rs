/*
 * @Date: 2024-07-08 23:25:28
 * @LastEditTime: 2024-07-09 11:19:51
 */
#![allow(unused_imports)]
#[cfg(target_arch = "wasm32")]
use async_std::task::sleep;
#[cfg(target_arch = "wasm32")]
use instant::{Duration, Instant};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;

pub async fn sleep_ms(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

pub async fn instant() -> Instant {
    Instant::now()
}
