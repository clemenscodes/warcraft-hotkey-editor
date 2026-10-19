mod model;
mod view;

pub use view::DialogFooterButtonView;
mod style;

use dioxus::prelude::*;
use model::DialogFooterButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DialogFooterButton(props: DialogFooterButtonModel) -> Element {
    let onclick = props.onclick;
    let label = props.label;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
        }
    }
}

assert_component!(DialogFooterButton);
