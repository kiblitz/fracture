use crate::command_chain;
use crate::vim_mode;

use std::sync::Mutex;

use dioxus::prelude::*;
use im::Vector;

pub struct Commander<F>
where
    F: FnMut(),
{
    leader_mapping: command_chain::map::Map<Mutex<F>>,
}

impl<F> Commander<F>
where
    F: FnMut(),
{
    pub fn new(f: F) -> Self {
        let leader_mapping = command_chain::map::Map::new()
            .with("  ".to_owned(), Mutex::new(f))
            .unwrap();
        Self { leader_mapping }
    }

    pub fn on_key_press(
        self: &Self,
        mode: Signal<vim_mode::Mode>,
        mut current_command_chain: Signal<Vector<char>>,
        c: char,
    ) {
        match mode() {
            vim_mode::Mode::Normal => {
                current_command_chain.write().push_back(c);
                match self.leader_mapping.search(current_command_chain()) {
                    command_chain::map::SearchResult::None => {
                        if current_command_chain().is_empty() {
                        } else {
                            current_command_chain.set(Vector::new())
                        };
                    }
                    command_chain::map::SearchResult::Value(f) => {
                        f.lock().unwrap()();
                        current_command_chain.set(Vector::new());
                    }
                    command_chain::map::SearchResult::Children(_) => (),
                }
            }
            vim_mode::Mode::Insert => (),
            vim_mode::Mode::Visual => (),
        }
    }
}
