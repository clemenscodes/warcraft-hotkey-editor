pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::toast_card::{ToastCard, ToastCardProps};
use dioxus::prelude::*;
pub use props::ToastListItemProps;
use style::CLASS;
assert_component!(ToastListItem);

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
