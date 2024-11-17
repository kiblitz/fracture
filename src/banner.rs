use dioxus::prelude::*;

#[component]
pub fn Banner() -> Element {
    rsx! {
        div { class: "relative w-full bg-gradient-to-tr from-blue-700 to-purple-400 text-white py-4 rounded-b-lg",
            { VimInfo() },
            { ModuleNavigator() },
            { FractureStatus() }
        }
    }
}

#[component]
fn VimInfo() -> Element {
    rsx! {
        div { class: "absolute left-0 top-0 h-full flex items-center px-2",
            p { "normal" }
        }
    }
}

#[component]
fn ModuleNavigator() -> Element {
    rsx! {
        div { class: "absolute left-0 right-0 top-0 h-full flex justify-center items-center",
            button { class: "bg-green-300 hover:bg-green-400 text-white rounded-full px-8",
                "module"
            }
        }
    }
}

#[component]
fn FractureStatus() -> Element {
    rsx! {
        div { class: "absolute right-0 top-0 h-full flex justify-center items-center px-2",
            p { class: "bg-orange-700 hover:bg-red-700 text-white rounded-lg px-2",
                "fracturing"
            }
        }
    }
}
