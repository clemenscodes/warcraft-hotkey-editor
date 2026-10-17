pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitDetailDialogView;

use crate::components::app::components::shell::components::shared::warcraft_dialog::WarcraftDialog;
use components::unit_detail_dialog_body::UnitDetailDialogBodyView;
use dioxus::prelude::*;
use dioxus_kit::frame::Empty;
use model::UnitDetailDialogModel;
use presentation::{ResolvedDialogUnit, UnitDetailDialogPresentation, use_unit_detail_dialog};
use tw_macro::assert_component;

#[component]
pub fn UnitDetailDialog(props: UnitDetailDialogModel) -> Element {
    let UnitDetailDialogPresentation {
        open,
        on_open_change,
        resolved,
    } = use_unit_detail_dialog(&props);
    let Some(resolved) = resolved else {
        return rsx! {};
    };
    let ResolvedDialogUnit {
        unit_name,
        portrait_url,
        description_text,
        combat,
        hero_attributes,
        evasion,
    } = resolved;
    let body = UnitDetailDialogBodyView {
        portrait_url,
        description_text,
        combat,
        hero_attributes,
        evasion,
    };
    rsx! {
        if open {
            WarcraftDialog::<UnitDetailDialogBodyView, Empty> {
                title: unit_name,
                body,
                open: true,
                on_open_change,
            }
        }
    }
}

assert_component!(UnitDetailDialog);
