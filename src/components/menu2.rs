use dioxus::prelude::*;
// use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct Menus {
    pub items: Vec<MenuItem>,
    pub on_click: EventHandler<String>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct MenuItem {
    pub key: String,
    pub icon: String,
    pub label: String,
    pub parent_key: Option<String>,
}

#[component]
pub fn Menu(cx: Menus) -> Element {
    let render_item = |item: &MenuItem| {
        let onclick = cx.on_click.clone();
        let key = item.key.clone();
        let children = find_children(&cx.items, &key);
        let mut menu_open = use_signal(|| vec![false, false]);

        rsx! {
            div {
                class: "flex items-center px-6 py-2 mt-4 duration-200 border-l-4 border-gray-900 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer",
                onclick: move |_| onclick.call(key.clone()),
                div {
                    class: "flex items-center space-x-4",
                    span { "{item.icon}" }, // 这里假设图标是一个字符串
                    span { "{item.label}" }
                    if !children.is_empty() {
                        div {
                            class: format!("{}", if menu_open()[0] { "rotate-180" } else { "" }),
                            icons::icon_up_down {}
                        }
                    }
                }
                if !children.is_empty() {
                    div {
                        class: format!("ml-8 mt-1 {}", if menu_open()[0] { "block" } else { "hidden" }),
                        {children}.iter().map(render_item).collect::<Vec<_>>()
                    }
                }
            }
        }
    };

    rsx! {
        nav {
            class: "mt-10",
            // {cx}.items.iter().filter(|item| item.parent_key.is_none()).map(render_item).collect::<Vec<_>>()
        }
    }
}

fn find_children<'a>(items: &'a [MenuItem], parent_key: &str) -> Vec<&'a MenuItem> {
    items
        .iter()
        .filter(|item| item.parent_key.as_ref().map_or(false, |key| key == parent_key))
        .collect()
}

fn build_tree(items: Vec<MenuItem>) -> Vec<MenuItem> {
    let mut map: HashMap<String, MenuItem> = HashMap::new();
    let mut roots: Vec<MenuItem> = Vec::new();

    for item in items {
        map.insert(item.key.clone(), item.clone());
    }

    for item in items {
        if let Some(parent_key) = &item.parent_key {
            if let Some(parent) = map.get_mut(parent_key) {
                if parent.children.is_none() {
                    parent.children = Some(Vec::new());
                }
                if let Some(children) = &mut parent.children {
                    children.push(item);
                }
            }
        } else {
            roots.push(item);
        }
    }

    roots
}

// 使用示例
// fn app(cx: Scope) -> Element {
//     let menu_items = use_state(cx, || vec![]);

//     use_effect(cx, (), |_| {
//         to_owned![menu_items];
//         async move {
//             let client = Client::new();
//             let response = client
//                 .get("https://your-api-endpoint.com/menu")
//                 .send()
//                 .await
//                 .unwrap()
//                 .json::<Vec<MenuItem>>()
//                 .await
//                 .unwrap();
//             let tree = build_tree(response);
//             menu_items.set(tree);
//         }
//     });

//     cx.render(rsx! {
//         Menu {
//             items: menu_items.get().clone(),
//             on_click: move |key| {
//                 // 处理点击事件
//             }
//         }
//     })
// }

mod icons {
    use dioxus::prelude::*;
    use dioxus_html_macro::html;
    pub fn icon_up_down() -> Element {
        html!(
            <svg class="w-3 h-3"
                fill="currentColor"
                view_box="0 0 12 12">
                <path d="M5.9 11.4L.5 6l1.4-1.4 4 4 4-4L11.3 6z"></path>
            </svg>
        )
    }
}

fn render_menu_item(item: MenuItem) -> Element {
    let children = item.children;
    let mut menu_open = use_signal(|| false);
    let toggle_menu = if menu_open() { "block" } else { "hidden" };
    tracing::info!("menu_open: {}", toggle_menu);
    rsx! {
        div{
            class:"flex items-center px-6 py-2 mt-4 duration-200 border-l-4 border-gray-900 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer ",

            // aria_controls="dropdown-example"
            "x-data": "{{ open: false }}",

            a{ id: "dropdown-example", class:"flex items-center space-x-4",
                href:{item.key},
                // onclick = { move|_| { menu_open.set(!menu_open())}}>
                { item.icon}
                span{ class:"mx-4", { item.label } }
            }

        }
        { if !children.is_empty() {
            rsx! {
                div{ class:"ml-8 mt-1 { toggle_menu }",
                    {children.into_iter().map(|item| render_menu_item(item)) }
                }
            }
        } else {
            rsx! {}
        }}
    }
}
