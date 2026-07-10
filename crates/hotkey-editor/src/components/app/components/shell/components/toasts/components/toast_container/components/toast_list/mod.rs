pub mod components;
mod props;
mod view;

pub use view::ToastListView;
mod style;

use components::toast_list_item::ToastListItem;
use dioxus::prelude::*;
use props::ToastListProps;
use style::CLASS;
use tw_macro::assert_component;

/// The ordered stack of live toasts, newest nearest the bottom edge.
#[component]
pub fn ToastList(props: ToastListProps) -> Element {
    let toasts = props.toasts;
    let on_remove = props.on_remove;
    rsx! {
        ol {
            class: CLASS,
            for record in toasts {
                ToastListItem { key: "{record.id()}", record, on_remove }
            }
        }
    }
}

assert_component!(ToastList);
