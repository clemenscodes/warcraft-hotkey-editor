pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::toast_list_item::ToastListItem;
use dioxus::prelude::*;
use logic::ToastListPresentation;
pub use props::ToastListProps;
use style::CLASS;
assert_component!(ToastList);

/// The ordered stack of live toasts, newest nearest the bottom edge.
#[component]
pub fn ToastList(props: ToastListProps) -> Element {
    let presentation = ToastListPresentation::from(&props);
    let items = presentation.items();
    rsx! {
        ol {
            class: CLASS,
            for item in items {
                ToastListItem { key: "{item.record.id()}", ..item }
            }
        }
    }
}
