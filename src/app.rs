use crate::banner;
use crate::commander;
use crate::types::key;
use crate::types::vim_mode;

use dioxus::prelude::*;
use im::Vector;

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

/*
let _ = web_sys::window()
    .unwrap()
    .alert_with_message("");
*/

#[component]
fn Home() -> Element {
    let mut show_search = use_signal(|| false);
    let commander = commander::Commander::new(move || show_search.set(!show_search()));
    let current_command_chain = use_signal(|| Vector::new());
    let mode = use_signal(|| vim_mode::Mode::Normal);
    let on_key_down = move |e: KeyboardEvent| {
        let key = match e.key() {
            Key::Character(s) => {
                if s.len() == 1 {
                    Some(key::Key::Char(s.chars().next().unwrap()))
                } else {
                    None
                }
            }
            Key::Escape => Some(key::Key::Esc),
            _ => None,
        };

        match key {
            Some(key) => commander.on_key_press(mode, current_command_chain, key),
            None => (),
        }
    };

    let mut count = use_signal(|| 0);
    rsx! {
        div { class: "w-full h-full", tabindex: 0, onkeydown: on_key_down,
            div {
                Search { show: show_search() }
                { banner::Banner(banner::BannerProps { mode: mode(),  command_chain: current_command_chain() }) },
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
fn Search(show: bool) -> Element {
    rsx! {
        if show {
            div { class: "z-10 fixed flex w-full h-full items-center justify-center pointer-events-none",
                div { class: "bg-zinc-50 opacity-75 p-10 rounded-lg shadow-lg pointer-events-auto",
                    p { "test" }
                }
            }
        }
    }
}
