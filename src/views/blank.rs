/*
 * @Author: plucky
 * @Date: 2022-10-15 09:52:33
 * @LastEditTime: 2022-10-15 09:58:23
 * @Description: 
 */

use dioxus::prelude::*;
use dioxus_html_macro::html;

pub fn view(cx: Scope)->Element{
    
    cx.render(html!{
        <h3 class="text-gray-700 text-3xl font-medium">"Blank Page"</h3>
        
    })
}
