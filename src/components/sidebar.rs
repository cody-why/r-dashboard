/*
 * @Author: plucky
 * @Date: 2022-10-11 18:53:17
 * @LastEditTime: 2022-10-17 09:52:40
 * @Description: 
 */

use dioxus::prelude::*;
use dioxus::prelude::Atom;

pub static IS_SIDEBAR_OPEN: Atom<bool> = |_| false;

pub fn view(cx:Scope)->Element{
    let route = use_route(&cx);
    let route_name = route.last_segment().unwrap_or_default();
    
    let set_sidebar_open = use_set(&cx,IS_SIDEBAR_OPEN);
    let is_sidebar_open = use_read(&cx,IS_SIDEBAR_OPEN);

    let is_menu_open = use_ref(&cx,||vec![false,false]);

    let highlight_class = |e:&str|{
        match e == route_name {
            true => "flex items-center px-6 py-2 mt-4 duration-200 border-l-4 bg-gray-600 bg-opacity-25 text-gray-100 border-gray-100",
            false => "flex items-center px-6 py-2 mt-4 duration-200 border-l-4 border-gray-900 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100",
        }
    };

    cx.render(rsx!(
        //  class: format_args!("flex  {} bg-gray-900  ",if *is_sidebar_open { "" } else { "hidden lg:block" }),
        // mask
        div{
            onclick: move |_| {
                set_sidebar_open(false);
            },
            class: format_args!("fixed inset-0 z-20 transition-opacity bg-black opacity-50 lg:hidden {}",if *is_sidebar_open {"block"} else {"hidden"}),
            // hidden: format_args!("{}",!is_sidebar_open),

        }

        // sidebar
        div{
            class: format_args!("fixed inset-y-0 left-0 z-30 w-56 overflow-y-auto transition duration-300 transform bg-gray-900 lg:translate-x-0 lg:static lg:inset-0 {}", 
            if *is_sidebar_open { "translate-x-0 ease-out " } else {"-translate-x-full ease-in"  }),
            
            // title
            div {
                class: "flex items-center justify-center mt-8 ",
                div {
                    class: "flex items-center",
                    icons::icon_1 {}
                    span {
                        class: "mx-2 text-2xl font-semibold text-white",
                        "R-Dashboard"
                    }
                }
            }
            // menu
            nav{
                class: "mt-10 ",//
                // router-link
                Link{
                    class: highlight_class("dashboard"),
                    //to="/dashboard"
                    to: "/dashboard",
                    icons::icon_2 {}
                    span{class:"mx-4","Dashboard"}
                }
                Link{
                    class: highlight_class("ui-elements"),
                    to: "/ui-elements",
                    icons::icon_3 {}
                    span{class:"mx-4","UI Elements"}
                }
                Link{
                    class: highlight_class("tables"),
                    to: "/tables",
                    icons::icon_4 {}
                    span{class:"mx-4","Tables"}
                }
                Link{
                    class: highlight_class("forms"),
                    to: "/forms",
                    icons::icon_5 {}
                    span{class:"mx-4","Forms"}
                }
                Link{
                    class: highlight_class("cards"),
                    to: "/cards",
                    icons::icon_6 {}
                    span{class:"mx-4","Cards"}
                }
                Link{
                    class: highlight_class("modal"),
                    to: "/modal",
                    icons::icon_7 {}
                    span{class:"mx-4","Modal"}
                }
                
                Link{
                    class: highlight_class("blank"),
                    to: "/blank",
                    icons::icon_8 {}
                    span{class:"mx-4","Blank"}
                }
                
                // ul{li{}}
                div{
                    div{
                        class: "flex items-center px-6 py-2 mt-4 duration-200 border-l-4 border-gray-900 text-gray-500 hover:bg-gray-600 hover:bg-opacity-25 hover:text-gray-100 cursor-pointer",
                        onclick: move |_| {
                            let mut is_menu_open = is_menu_open.write();
                            is_menu_open[0] = !is_menu_open[0];
                        },

                        div {
                            class: "flex items-center space-x-4",
                            icons::icon_2 {}
                            span {
                                "Test"
                            }
                            div{
                                class: format_args!{"{}",if is_menu_open.read()[0] {"rotate-180"} else {""}} ,
                                icons::icon_up_down {}
                            }
                        }
                    }
                    
                    div{
                        class: format_args!("ml-8 mt-1 {}", if is_menu_open.read()[0] {"block"} else {"hidden"}),
                        Link{
                            class: highlight_class("blank"),
                            to: "/blank",
                            icons::icon_8 {}
                            span{class:"mx-4","Blank"}
                        }
                        Link{
                            class: highlight_class("blank2"),
                            to: "/blank",
                            icons::icon_8 {}
                            span{class:"mx-4","Blank"}
                        }
                    }
                    
                }
                //  end

            }

            

        }
    ))
    
}

mod icons{
    use dioxus::prelude::*;
    use dioxus_html_macro::html;

    pub fn icon_1(cx: Scope)->Element{
        cx.render(html!{
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
        })
    }
    pub fn icon_2(cx:Scope)->Element{
        cx.render(html!{
            <svg
                class="w-5 h-5"
                view_box="0 0 20 20"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M2 10C2 5.58172 5.58172 2 10 2V10H18C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10Z"
                  fill="currentColor"
                />
                <path
                  d="M12 2.25195C14.8113 2.97552 17.0245 5.18877 17.748 8.00004H12V2.25195Z"
                  fill="currentColor"
                />
            </svg>
        })
    }

    pub fn icon_3(cx:Scope)->Element{
        cx.render(html!(
            <svg
                class="w-5 h-5"
                view_box="0 0 20 20"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M5 3C3.89543 3 3 3.89543 3 5V7C3 8.10457 3.89543 9 5 9H7C8.10457 9 9 8.10457 9 7V5C9 3.89543 8.10457 3 7 3H5Z"
                  fill="currentColor"
                />
                <path
                  d="M5 11C3.89543 11 3 11.8954 3 13V15C3 16.1046 3.89543 17 5 17H7C8.10457 17 9 16.1046 9 15V13C9 11.8954 8.10457 11 7 11H5Z"
                  fill="currentColor"
                />
                <path
                  d="M11 5C11 3.89543 11.8954 3 13 3H15C16.1046 3 17 3.89543 17 5V7C17 8.10457 16.1046 9 15 9H13C11.8954 9 11 8.10457 11 7V5Z"
                  fill="currentColor"
                />
                <path
                  d="M11 13C11 11.8954 11.8954 11 13 11H15C16.1046 11 17 11.8954 17 13V15C17 16.1046 16.1046 17 15 17H13C11.8954 17 11 16.1046 11 15V13Z"
                  fill="currentColor"
                />
            </svg>
        ))
    }

    pub fn icon_4(cx:Scope)->Element{
        cx.render(html!(
            <svg
                class="w-5 h-5"
                view_box="0 0 20 20"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  d="M7 3C6.44772 3 6 3.44772 6 4C6 4.55228 6.44772 5 7 5H13C13.5523 5 14 4.55228 14 4C14 3.44772 13.5523 3 13 3H7Z"
                  fill="currentColor"
                />
                <path
                  d="M4 7C4 6.44772 4.44772 6 5 6H15C15.5523 6 16 6.44772 16 7C16 7.55228 15.5523 8 15 8H5C4.44772 8 4 7.55228 4 7Z"
                  fill="currentColor"
                />
                <path
                  d="M2 11C2 9.89543 2.89543 9 4 9H16C17.1046 9 18 9.89543 18 11V15C18 16.1046 17.1046 17 16 17H4C2.89543 17 2 16.1046 2 15V11Z"
                  fill="currentColor"
                />
            </svg>
        ))
    }

    pub fn icon_5(cx:Scope)->Element{
        cx.render(html!(
            <svg class="w-5 h-5" fill="currentColor" view_box="0 0 20 20">
                <path
                  d="M17.414 2.586a2 2 0 00-2.828 0L7 10.172V13h2.828l7.586-7.586a2 2 0 000-2.828z"
                />
                <path
                  fill_rule="evenodd"
                  d="M2 6a2 2 0 012-2h4a1 1 0 010 2H4v10h10v-4a1 1 0 112 0v4a2 2 0 01-2 2H4a2 2 0 01-2-2V6z"
                  clip_rule="evenodd"
                />
            </svg>
        ))
    }

    pub fn icon_6(cx:Scope)->Element{
        cx.render(html!(
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" view_box="0 0 20 20" fill="currentColor">
                <path d="M4 4a2 2 0 00-2 2v1h16V6a2 2 0 00-2-2H4z" />
                <path fill_rule="evenodd" d="M18 9H2v5a2 2 0 002 2h12a2 2 0 002-2V9zM4 13a1 1 0 011-1h1a1 1 0 110 2H5a1 1 0 01-1-1zm5-1a1 1 0 100 2h1a1 1 0 100-2H9z" 
                clip_rule="evenodd" />
            </svg>
        ))
    }

    pub fn icon_7(cx:Scope)->Element{
        cx.render(html!(
            <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5" view_box="0 0 20 20" fill="currentColor">
              <path d="M3 12v3c0 1.657 3.134 3 7 3s7-1.343 7-3v-3c0 1.657-3.134 3-7 3s-7-1.343-7-3z" />
              <path d="M3 7v3c0 1.657 3.134 3 7 3s7-1.343 7-3V7c0 1.657-3.134 3-7 3S3 8.657 3 7z" />
              <path d="M17 5c0 1.657-3.134 3-7 3S3 6.657 3 5s3.134-3 7-3 7 1.343 7 3z" />
            </svg>
        ))
    }

    pub fn icon_8(cx:Scope)->Element{
        cx.render(html!(
            <svg class="w-5 h-5" fill="currentColor" view_box="0 0 20 20">
                <path
                  d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"
                />
            </svg>
        ))
    }

    // #[inline_props]
    pub fn icon_up_down(cx:Scope)->Element{
        cx.render(html!(
            <svg class="w-3 h-3"
                fill="currentColor"
                view_box="0 0 12 12">
                <path d="M5.9 11.4L.5 6l1.4-1.4 4 4 4-4L11.3 6z"></path>
            </svg>
        ))
    }
}

