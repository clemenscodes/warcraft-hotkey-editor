use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeaderLayoutSlotProps {
    pub layout_dialog_open: Signal<bool>,
}
