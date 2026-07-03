pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::toast_list::{ToastList, ToastListProps};
use dioxus::prelude::*;
pub use props::ToastContainerProps;
use style::CLASS;
assert_component!(ToastContainer);

/// The fixed overlay anchoring the toast stack to the bottom-right of the
/// viewport. Click-through itself; each toast re-enables pointer events.
#[component]
pub fn ToastContainer(props: ToastContainerProps) -> Element {
    let list_props = ToastListProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            role: "region",
            "aria-label": "notifications",
            tabindex: "-1",
            ToastList { ..list_props }
        }
    }
}
