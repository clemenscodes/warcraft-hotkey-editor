pub mod components;
mod props;
mod style;

use components::toast_card::{ToastCard, ToastCardProps};
use dioxus::prelude::*;
pub use props::ToastListItemProps;
use style::CLASS;
use tw_macro::assert_component;

/// A single list slot. Re-enables pointer events (the container is click-through)
/// and hosts one toast card.
#[component]
pub fn ToastListItem(props: ToastListItemProps) -> Element {
    let card_props = ToastCardProps::from(&props);
    rsx! {
        li {
            class: CLASS,
            ToastCard { ..card_props }
        }
    }
}

assert_component!(ToastListItem);
