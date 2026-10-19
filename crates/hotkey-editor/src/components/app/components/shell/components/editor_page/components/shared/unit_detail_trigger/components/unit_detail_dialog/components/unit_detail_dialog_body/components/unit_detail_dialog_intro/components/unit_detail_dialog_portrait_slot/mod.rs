mod model;
mod view;

pub use view::UnitDetailDialogPortraitSlotView;
mod style;

use dioxus::prelude::*;
use model::UnitDetailDialogPortraitSlotModel;
use style::CLASS;
use tw_macro::assert_component;

use super::super::super::super::super::super::super::shared::unit_detail_portrait::UnitDetailPortrait;

#[component]
pub fn UnitDetailDialogPortraitSlot(props: UnitDetailDialogPortraitSlotModel) -> Element {
    let portrait_url = props.portrait_url;
    rsx! {
        div {
            class: CLASS,
            UnitDetailPortrait {
                src: portrait_url,
            }
        }
    }
}

assert_component!(UnitDetailDialogPortraitSlot);
