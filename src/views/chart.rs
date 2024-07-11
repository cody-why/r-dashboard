/*
 * @Date: 2024-07-08 16:13:27
 * @LastEditTime: 2024-07-11 09:09:54
 */
use dioxus::prelude::*;

use crate::utils::time::sleep_ms;

pub fn eval_chart() {
    eval_chart1();
    eval_chart2();
}
pub fn eval_chart1() {
    let script = include_str!("js/chart1.js");
    let _eval = eval(script);

    spawn(async move {
        sleep_ms(1000).await;
        _eval.send((vec![148, 150, 130, 170]).into()).unwrap();
    });
}

pub fn eval_chart2() {
    let script = include_str!("js/chart2.js");
    let _eval = eval(script);
}
