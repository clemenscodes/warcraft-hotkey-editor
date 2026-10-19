pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitDetailTriggerView;
mod style;

use components::unit_detail_button::UnitDetailButton;
use components::unit_detail_dialog::UnitDetailDialog;
use dioxus::prelude::*;
use model::UnitDetailTriggerModel;
use presentation::{UnitDetailTriggerPresentation, use_unit_detail_trigger};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetailTrigger(props: UnitDetailTriggerModel) -> Element {
    let UnitDetailTriggerPresentation {
        icon_url,
        unit_id,
        open,
        onpointerdown,
        onclick,
        on_open_change,
    } = use_unit_detail_trigger(&props);
    rsx! {
        div {
            class: CLASS,
            onpointerdown,
            UnitDetailButton {
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

assert_component!(UnitDetailTrigger);
