pub mod components;
mod model;
mod view;

pub use view::UnitDetailDialogIntroView;
mod style;

use components::unit_description::UnitDescription;
use components::unit_detail_dialog_portrait_slot::UnitDetailDialogPortraitSlot;
use dioxus::prelude::*;
use model::UnitDetailDialogIntroModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetailDialogIntro(props: UnitDetailDialogIntroModel) -> Element {
    let portrait_url = props.portrait_url;
    let description_text = props.description_text;
    rsx! {
        div {
            class: CLASS,
            UnitDetailDialogPortraitSlot {
                portrait_url,
            }
            UnitDescription {
                text: description_text,
            }
        }
    }
}

assert_component!(UnitDetailDialogIntro);
