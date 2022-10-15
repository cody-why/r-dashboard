/*
 * @Author: plucky
 * @Date: 2022-10-11 00:07:29
 * @LastEditTime: 2022-10-15 12:38:33
 * @Description: 
 */
#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::{components::*, views::*};

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            self::routes {}
        }
    })
}

/// Routes
fn routes(cx: Scope) -> Element {
    cx.render(rsx! {
        Route { to: "/", login::view{}}
        Route { to: "/dashboard", Home{} }
        Route { to: "/ui-elements", Home{} }
        Route { to: "/tables", Home{} }
        Route { to: "/forms", Home{} }
        Route { to: "/cards", Home{} }
        Route { to: "/modal", Home{} }
        Route { to: "/blank", Home{} }
        
        Redirect {from: "", to: "/" }
    })
}

/// Home Page View 
fn Home(cx: Scope)->Element{
    let route = use_route(&cx);

    let h = route.last_segment().unwrap_or_default();

    cx.render(rsx!{
        div {
            class: "flex h-screen bg-gray-200 font-roboto",
            sidebar::view{}
            div {
                class: "flex-1 flex flex-col overflow-hidden",
                header::view{}
                main {
                    class: "flex-1 overflow-x-hidden overflow-y-auto bg-gray-200",
                    div {
                        class: "container mx-auto px-6 py-8",
                        slot {
                            i{"{h}"}
                            match h {
                                "dashboard" => rsx!{ dashboard::view{} },
                                "ui-elements" => rsx!{ ui_elements::view{} },
                                "tables" => rsx!{tables::view{} },
                                "forms" => rsx!{forms::view{} },
                                "cards" => rsx!{card::view{} },
                                "modal" => rsx!{modal::view{} },
                                "blank" => rsx!{blank::view{} },
                                _ => rsx!{ div{}}
                            }
                           
                        }
                    }
                }
            }
        }
       
    })
}


// pub fn app2(cx: Scope)->Element{
//     let show_title = true;
//     let user_name = Some("bob");
//     cx.render(rsx!{
//         div {
//             class: format_args!("text-2xl font-bold {}", show_title.then(|| "text-red-500").unwrap_or("hidden")),
//             // Renders nothing by returning None when show_title is false
//             show_title.then(|| rsx!{
//                 "This is the title"
//             }),
//         }

//         div {
//             // Renders nothing if user_name is None
//             user_name.map(|name| rsx!("Hello {name}"))
//         }
//     })
// }

// router 参数解析
// #[derive(Deserialize)]
// struct Query { name: String }
// // blogpost:post?name=plucky
// fn BlogPost(cx: Scope) -> Element {
//     let post = use_route(&cx).segment("post")?;
//     let query = use_route(&cx).query::<Query>()?;

//     cx.render(rsx!{
//         "Viewing post {post}"
//         "Name selected: {query}"
//     })
// }


