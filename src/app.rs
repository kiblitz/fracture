use crate::banner;

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let on_key_down = move |e: KeyboardEvent| match e.key() {
        Key::Character(s) => {
            if s.len() == 1 {
                let _ = web_sys::window().unwrap().alert_with_message(&s.to_owned());
            }
        }
        _ => (),
    };

    rsx! {
        div { class: "w-full h-full", tabindex: 0, onkeydown: on_key_down,
            div {
                { Search() },
                { banner::Banner() },
                h1 { "High-Five counter: {count}" }
                Link { to: Route::Blog { id: count() }, "Go to blog" }
                button {
                    class: "bg-blue-300 hover:bg-blue-700 text-white rounded-full px-4 py-2",
                    onclick: move |_| count += 1,
                    "Up high!"
                }
                button { onclick: move |_| count -= 1, "Down low!" }
            }
            div { { Editor() } }
        }
    }
}

#[component]
fn Editor() -> Element {
    let content = use_signal(|| "test".to_owned());
    rsx! {
        div {
            button { { content() } }
        }
    }
}

#[component]
fn Search() -> Element {
    rsx! {
        div { class: "z-10 fixed flex w-full h-full items-center justify-center pointer-events-none",
            div { class: "bg-zinc-50 opacity-75 p-10 rounded-lg shadow-lg pointer-events-auto",
                p { "test" }
            }
        }
    }
}
