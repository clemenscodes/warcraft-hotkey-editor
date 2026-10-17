pub mod components;
mod model;
mod presentation;
mod view;

pub use view::PagerCardDetailTriggerView;
mod style;

use components::pager_card_detail_button::PagerCardDetailButton;
use components::unit_detail_dialog::UnitDetailDialog;
use dioxus::prelude::*;
use model::PagerCardDetailTriggerModel;
use presentation::{PagerCardDetailTriggerPresentation, use_pager_card_detail_trigger};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCardDetailTrigger(props: PagerCardDetailTriggerModel) -> Element {
    let PagerCardDetailTriggerPresentation {
        icon_url,
        unit_id,
        open,
        onclick,
        on_open_change,
    } = use_pager_card_detail_trigger(&props);
    rsx! {
        div {
            class: CLASS,
            PagerCardDetailButton {
                src: icon_url,
                onclick,
            }
            UnitDetailDialog {
                unit_id,
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(PagerCardDetailTrigger);
