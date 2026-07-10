pub mod components;
mod props;
mod view;

pub use view::ToastContainerView;
mod style;

use components::toast_list::ToastList;
use dioxus::prelude::*;
use props::ToastContainerProps;
use style::CLASS;
use tw_macro::assert_component;

/// The fixed overlay anchoring the toast stack to the bottom-right of the
/// viewport. Click-through itself; each toast re-enables pointer events.
#[component]
pub fn ToastContainer(props: ToastContainerProps) -> Element {
    let toasts = props.toasts;
    let on_remove = props.on_remove;
    rsx! {
        div {
            class: CLASS,
            role: "region",
            "aria-label": "notifications",
            tabindex: "-1",
            ToastList { toasts, on_remove }
        }
    }
}

assert_component!(ToastContainer);
