mod model;
mod view;

pub use view::PagerCardDetailButtonView;
mod style;

use super::shared::pager_card_portrait::PagerCardPortrait;
use dioxus::prelude::*;
use model::PagerCardDetailButtonModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCardDetailButton(props: PagerCardDetailButtonModel) -> Element {
    let src = props.src;
    let onclick = props.onclick;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            aria_label: "Show unit details",
            aria_haspopup: "dialog",
            onclick,
            PagerCardPortrait {
                src,
            }
        }
    }
}

assert_component!(PagerCardDetailButton);
