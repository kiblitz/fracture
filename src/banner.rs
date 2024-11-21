use crate::types::vim_mode;

use dioxus::prelude::*;
use im::Vector;

#[component]
pub fn Banner(mode: vim_mode::Mode, command_chain: Vector<char>) -> Element {
    rsx! {
        div { class: "relative w-full bg-gradient-to-tr from-blue-700 to-purple-400 text-white py-4 rounded-b-lg",
            { VimInfo(VimInfoProps { mode, command_chain }) },
            { ModuleNavigator() },
            { FractureStatus() }
        }
    }
}

#[component]
fn VimInfo(mode: vim_mode::Mode, command_chain: Vector<char>) -> Element {
    let status = format!("[{}] {}", mode, command_chain_to_string(command_chain));
    rsx! {
        div { class: "absolute left-0 top-0 h-full flex items-center px-2",
            p { "{status}" }
        }
    }
}

fn command_chain_to_string(command_chain: Vector<char>) -> String {
    let mut str = String::new();
    command_chain
        .iter()
        .flat_map(|c| match c {
            ' ' => [" ".to_owned(), "<SPC>".to_owned()],
            c => [" ".to_owned(), c.to_string()],
        })
        .skip(1)
        .for_each(|s| str.push_str(&s));
    str
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
