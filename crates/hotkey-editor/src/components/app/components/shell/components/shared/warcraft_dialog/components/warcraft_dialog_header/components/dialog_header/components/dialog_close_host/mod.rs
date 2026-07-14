pub mod components;
mod model;
mod view;

pub use view::DialogCloseHostView;
mod style;

use components::dialog_close::DialogClose;
use dioxus::prelude::*;
use model::DialogCloseHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DialogCloseHost(props: DialogCloseHostModel) -> Element {
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            DialogClose {
                onclick,
            }
        }
    }
}

assert_component!(DialogCloseHost);
