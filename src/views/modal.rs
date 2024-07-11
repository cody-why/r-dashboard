/*
* @Date: 2022-10-15 09:52:14
 * @LastEditTime: 2024-07-09 16:42:32
* @Description:
*/

use dioxus::prelude::*;

pub fn view() -> Element {
    let mut open = use_signal(|| false);
    // let modal = "transition: opacity 0.25s ease";

    rsx! {
        div {
            h3 { class: "text-3xl font-medium text-gray-700", "Modal" }
            // open modal
            button {
                onclick: move |_| {
                    open.set(true);
                },
                class: "px-6 py-3 mt-3 font-medium tracking-wide text-white bg-indigo-600 rounded-md hover:bg-indigo-500 focus:outline-none",
                "Open Modal"
            }

            // Modal
            div {
                class: format!(
                    "transition:opacity 0.25s ease {} z-50 fixed w-full h-full top-0 left-0 flex items-center justify-center",
                    if open() { "" } else { "opacity-0 pointer-events-none" },
                ),
                // overlay
                div { class: "absolute w-full h-full bg-gray-900 opacity-50 modal-overlay" }

                //
                div { class: "z-50 w-11/12 mx-auto overflow-y-auto bg-white rounded shadow-lg modal-container md:max-w-md",
                    // 右上角关闭按钮
                    div {
                        onclick: move |_| {
                            open.set(false);
                        },
                        class: "absolute top-0 right-0 z-50 flex flex-col items-center mt-4 mr-4 text-sm text-white cursor-pointer modal-close",
                        icons::icon_1 {}
                        span { class: "text-sm", "(Esc)" }
                    }

                    //<!-- Add margin if you want to see some of the overlay behind the modal-->
                    div { class: "px-6 py-4 text-left modal-content",
                        //<!--Title-->
                        div { class: "flex items-center justify-between pb-3",
                            p { class: "text-2xl font-bold", "Modal Title" }
                            div {
                                class: "z-50 cursor-pointer modal-close",
                                onclick: move |_| {
                                    open.set(false);
                                },
                                icons::icon_1 {}
                            }
                        }

                        //<!--Body-->
                        p { "Modal content." }

                        //<!--Footer-->
                        div { class: "flex justify-end pt-2",
                            button {
                                onclick: move |_| {
                                    open.set(false);
                                },
                                class: "p-3 px-6 py-3 mr-2 text-indigo-500 bg-transparent rounded-lg hover:bg-gray-100 hover:text-indigo-400 focus:outline-none",
                                "Close"
                            }
                            button {
                                onclick: move |_| {
                                    open.set(false);
                                },
                                class: "px-6 py-3 font-medium tracking-wide text-white bg-indigo-600 rounded-md hover:bg-indigo-500 focus:outline-none",
                                "Action"
                            }
                        }
                    }
                }
            }
        }
    }
}

mod icons {
    use dioxus::prelude::*;
    use dioxus_html_macro::html;

    pub fn icon_1() -> Element {
        html! {
            <svg
                class="w-5 h-5"
                fill="none"
                view_box="0 0 24 24"
                stroke="currentColor"
            >
                <path
                stroke_linecap="round"
                stroke_linejoin="round"
                stroke_width="2"
                d="M6 18L18 6M6 6l12 12"
                />
            </svg>
        }
    }
}
