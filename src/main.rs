/*
 * @Date: 2022-10-10 23:58:17
 * @LastEditTime: 2024-07-08 23:28:50
 * @Description:
 */
pub mod app;
pub mod components;
pub mod modules;
pub mod utils;
pub mod views;

use app::App;

// trunk serve
fn main() {
    console_error_panic_hook::set_once();
    let mut cfg = tracing_wasm::WASMLayerConfigBuilder::default();
    cfg.set_max_level(tracing::Level::DEBUG);
    tracing_wasm::set_as_global_default_with_config(cfg.build());
    dioxus::launch(App);
}
