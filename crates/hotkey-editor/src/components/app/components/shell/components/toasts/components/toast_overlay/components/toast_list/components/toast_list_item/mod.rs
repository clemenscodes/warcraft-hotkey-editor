pub mod components;
mod model;
mod view;

pub use view::ToastListItemView;
mod style;

use components::toast::Toast;
use dioxus::prelude::*;
use model::ToastListItemModel;
use style::CLASS;
use tw_macro::assert_component;

/// A single list slot. Re-enables pointer events (the container is click-through)
/// and hosts one toast card.
#[component]
pub fn ToastListItem(props: ToastListItemModel) -> Element {
    let record = props.record;
    let on_remove = props.on_remove;
    rsx! {
        li {
            class: CLASS,
            Toast {
                record,
                on_remove,
            }
        }
    }
}

assert_component!(ToastListItem);
