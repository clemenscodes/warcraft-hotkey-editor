mod model;
mod view;

pub use view::UnitDetailButtonView;
mod style;

use super::shared::unit_detail_portrait::UnitDetailPortrait;
use dioxus::prelude::*;
use model::UnitDetailButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetailButton(props: UnitDetailButtonModel) -> Element {
    let src = props.src;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Show unit details",
            aria_haspopup: "dialog",
            onclick,
            UnitDetailPortrait {
                src,
            }
        }
    }
}

assert_component!(UnitDetailButton);
