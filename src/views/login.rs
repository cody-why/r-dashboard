/*
* @Date: 2022-10-12 00:00:54
 * @LastEditTime: 2024-07-11 09:54:59
* @Description:
*/
use dioxus::prelude::*;

pub fn view() -> Element {
    let mut email = use_signal(|| "".to_string());
    let mut password = use_signal(|| "".to_string());
    let mut email_error = use_signal(|| None::<String>);
    let mut password_error = use_signal(|| None::<String>);

    fn login() {
        let router = router();
        router.push("/home");
    }

    rsx! {
        div { class: "flex items-center justify-center h-screen px-6 bg-gray-200",
            div { class: "w-full max-w-sm p-6 bg-white rounded-md shadow-md",
                div { class: "flex items-center justify-center",
                    icons::icon_1 {}
                    span { class: "text-2xl font-semibold text-gray-700", "R-Dashboard" }
                }

                form {
                    class: "mt-4",
                    onsubmit: move |_| {
                        email_error.set(if email().is_empty() { Some("Email is required".to_string()) } else { None });
                        password_error.set(if password().is_empty() { Some("Password is required".to_string())} else {None});
                        if email().is_empty() || password().is_empty() {
                            // return;
                        }
                        login();
                    },
                    label { class: "block",
                        span { class: "text-sm text-gray-700", "Email" }
                        input {
                            class: "block w-full mt-1 c-input",
                            r#type: "email",
                            placeholder: "Enter your email",
                            value: "{email}",
                            oninput: move |e| {
                                email.set(e.value());
                            }
                        }
                        {email_error().map(|error| {
                            rsx! {
                                p { class: "mt-2 text-sm text-red-600", "{error}" }
                            }
                        })}
                    }
                    label { class: "block mt-3",
                        span { class: "text-sm text-gray-700", "Password" }
                        input {
                            class: "block w-full mt-1 c-input",
                            r#type: "password",
                            placeholder: "Enter your password",
                            value: "{password}",
                            oninput: move |e| {
                                password.set(e.value());
                            }
                        }
                        {password_error().map(|error| {
                            rsx! {
                                p { class: "mt-2 text-sm text-red-600", "{error}" }
                            }
                        })}
                    }
                    div { class: "flex items-center justify-between mt-4",
                        div {
                            label { class: "inline-flex items-center",
                                input {
                                    class: "text-indigo-600 c-input",
                                    r#type: "checkbox"
                                }
                                span { class: "mx-2 text-sm text-gray-600", "Remember me" }
                            }
                        }
                        div {
                            a {
                                class: "block text-sm text-indigo-700 fontme hover:underline",
                                href: "#",
                                "Forgot your password?"
                            }
                        }
                    }
                    div { class: "mt-6",
                        button {
                            class: "w-full px-4 py-2 text-sm text-center text-white bg-indigo-600 rounded-md focus:outline-none hover:bg-indigo-500",
                            r#type: "submit",
                            "Sign in"
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
                class="w-10 h-10"
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
