/*
 * @Date: 2022-10-11 18:53:17
 * @LastEditTime: 2024-07-11 10:48:17
 * @Description:
 */

use dioxus::prelude::*;

use crate::{components::menu, modules::demo_data::MENUS};

use super::SIDEBAR_OPEN;

pub fn view() -> Element {
    // 共享状态
    let mut sidebar_open = use_hook(|| SIDEBAR_OPEN.signal());

    let menus = use_hook(|| MENUS.signal());

    let toggle_sidebar = if sidebar_open() {
        "translate-x-0 ease-out"
    } else {
        "-translate-x-full ease-in"
    };

    rsx!(
        // this mask click then sidebar will be closed
        div {
            onmouseover: move |_| {
                sidebar_open.set(false);
            },
            class: "fixed inset-0 left-14 z-20 bg-black opacity-10 lg:hidden",
            hidden: !sidebar_open()
        }

        // sidebar
        div {
            class: "fixed inset-y-0 left-0 z-30 w-50 overflow-y-auto transition duration-300 bg-gray-900 lg:translate-x-0 lg:static lg:inset-0 {toggle_sidebar}",

            // title
            div { class: "flex items-center justify-center mt-8 ",
                div { class: "flex items-center",
                    icons::icon_logo {}
                    span { class: "hidden lg:block mx-2 text-2xl font-semibold text-white", "Dashboard" }
                }
            }
            // menu
            {menu::View(menus())}

        }
    )
}

mod icons {
    use dioxus::prelude::*;
    use dioxus_html_macro::html;

    pub fn icon_logo() -> Element {
        html! {
            <svg
                class="w-12 h-12"
                view_box="0 0 512 512"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M364.61 390.213C304.625 450.196 207.37 450.196 147.386 390.213C117.394 360.22 102.398 320.911 102.398 281.6C102.398 242.291 117.394 202.981 147.386 172.989C147.386 230.4 153.6 281.6 230.4 307.2C230.4 256 256 102.4 294.4 76.7999C320 128 334.618 142.997 364.608 172.989C394.601 202.981 409.597 242.291 409.597 281.6C409.597 320.911 394.601 360.22 364.61 390.213Z"
                  fill="#4C51BF"
                  stroke="#4C51BF"
                  stroke_width="2"
                  stroke_linecap="round"
                  stroke_linejoin="round"
                />
                <path
                  d="M201.694 387.105C231.686 417.098 280.312 417.098 310.305 387.105C325.301 372.109 332.8 352.456 332.8 332.8C332.8 313.144 325.301 293.491 310.305 278.495C295.309 263.498 288 256 275.2 230.4C256 243.2 243.201 320 243.201 345.6C201.694 345.6 179.2 332.8 179.2 332.8C179.2 352.456 186.698 372.109 201.694 387.105Z"
                  fill="white"
                />
            </svg>
        }
    }
}
