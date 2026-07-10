pub mod components;
mod props;
mod style;

use components::toast_card::ToastCard;
use dioxus::prelude::*;
use props::ToastListItemProps;
use style::CLASS;
use tw_macro::assert_component;

/// A single list slot. Re-enables pointer events (the container is click-through)
/// and hosts one toast card.
#[component]
pub fn ToastListItem(props: ToastListItemProps) -> Element {
    let record = props.record;
    let on_remove = props.on_remove;
    rsx! {
        li {
            class: CLASS,
            ToastCard { record, on_remove }
        }
    }
}

assert_component!(ToastListItem);
