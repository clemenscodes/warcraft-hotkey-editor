pub mod components;
mod model;
mod view;

pub use view::ToastCloseHostView;
mod style;

use components::toast_close::ToastClose;
use dioxus::prelude::*;
use model::ToastCloseHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ToastCloseHost(props: ToastCloseHostModel) -> Element {
    let id = props.id;
    let on_remove = props.on_remove;
    rsx! {
        div {
            class: CLASS,
            ToastClose {
                id,
                on_remove,
            }
        }
    }
}

assert_component!(ToastCloseHost);
