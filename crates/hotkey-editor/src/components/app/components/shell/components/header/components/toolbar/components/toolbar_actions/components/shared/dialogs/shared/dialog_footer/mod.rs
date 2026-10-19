pub mod components;
mod model;
mod style;
mod view;

pub use view::DialogFooterView;

use components::dialog_footer_button::DialogFooterButton;
use dioxus::prelude::*;
use model::DialogFooterModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DialogFooter(props: DialogFooterModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    rsx! {
        footer {
            class: CLASS,
            DialogFooterButton {
                onclick,
                label,
            }
        }
    }
}

assert_component!(DialogFooter);
