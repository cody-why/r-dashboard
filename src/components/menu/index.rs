/*
 * @Date: 2024-07-09 22:37:33
 * @LastEditTime: 2024-07-11 09:37:45
 */
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_html_macro::html;

use crate::components::menu::icons;

use super::MenuItem;

pub fn View(menus: Vec<MenuItem>) -> Element {
    html! {
        <nav class="mt-10">
            {menus.into_iter().map(render_menu_item)}
        </nav>
    }
}

fn render_menu_item(item: MenuItem) -> Element {
    #[allow(deprecated)]
    let route = use_router();
    let route_name = route.current_route_string();

    let mut menu_open = use_signal(|| false);
    let toggle_menu = if menu_open() { "block" } else { "hidden" };

    let highlight_class = |e: &str| {
        match e == route_name {
            true => "flex items-center px-6 py-2 mt-2 duration-200 border-l-4 bg-gray-600 bg-opacity-25 text-gray-100 border-gray-100",
            false => "flex items-center px-6 py-2 mt-2 duration-200 border-l-4 border-gray-900 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100",
        }
    };

    let children = item.children;
    html! {
        <Link class = {highlight_class(&item.key)}
            to={&item.key}
            onclick = { move|_| { menu_open.set(!menu_open())}}
            >
            { item.icon}
            <span class="mx-4">{ item.label }</span>
            {if !children.is_empty()  {
                html! {
                    <div class = {format!("{}", if menu_open() { "rotate-180 mt-1" } else { "" })}>
                        {icons::icon_up_down()}
                    </div>
                }
            }else {
                html! {}
            }}
        </Link>
        { if !children.is_empty() {
            html! {
                <div class="ml-8 mt-1 { toggle_menu }">
                    {children.into_iter().map(render_menu_item) }
                </div>
            }
        } else {
            html! {}
        }}

    }
}
