/*
 * @Date: 2022-10-11 00:07:29
 * @LastEditTime: 2024-07-06 18:46:11
 * @Description:
 */
#![allow(non_snake_case)]

use crate::{components::*, views::*};
use dioxus::prelude::*;

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[route("/")]
    Login {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

fn Login() -> Element {
    login::view()
}

// fn Dashboard() -> Element {
//     let router = router();
//     let url = router.current_route_string();
//     tracing::warn!("url: {}", url);
//     Home("dashboard")
// }

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    tracing::info!("segments: {:?}", segments);
    if let Some(url) = segments.get(0) {
        Body(url)
    } else {
        Body("dashboard")
    }
}

// Home Page View
fn Body(url: impl AsRef<str>) -> Element {
    let url = url.as_ref();
    rsx! {
        div { class: "flex h-screen bg-gray-200 font-roboto",
            sidebar::view {}
            div { class: "flex-1 flex flex-col overflow-hidden w-full ",
                header::view {}
                main { class: "flex-1 overflow-x-hidden overflow-y-auto ",
                    div { class: "container mx-auto px-6 py-8",
                        slot {
                            i { "/{url}" }
                            match url {
                                "dashboard" | "home" => rsx!{dashboard::view{} },
                                "ui-elements" => rsx!{ui_elements::view{} },
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
    }
}
