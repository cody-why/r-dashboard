/*
 * @Author: plucky
 * @Date: 2022-10-10 23:58:17
 * @LastEditTime: 2022-10-15 14:02:24
 * @Description: 
 */

use r_dashboard::app::*;


// trunk serve
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    dioxus::web::launch(app);
}
