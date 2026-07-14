use dioxus::prelude::*;

pub mod context;

#[derive(Clone, Copy, PartialEq)]
pub struct ResolveSelection {
    selected_move_category: Signal<Option<String>>,
}

impl ResolveSelection {
    pub fn new(selected_move_category: Signal<Option<String>>) -> Self {
        Self {
            selected_move_category,
        }
    }

    pub fn selected_move_category(&self) -> Signal<Option<String>> {
        self.selected_move_category
    }
}
