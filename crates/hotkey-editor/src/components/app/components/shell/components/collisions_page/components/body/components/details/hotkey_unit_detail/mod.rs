pub mod components;
mod model;
mod view;

pub use view::HotkeyUnitDetailView;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::detail_card::DetailCard;
use components::hotkey_unit_detail_body::HotkeyUnitDetailBodyView;
use dioxus::prelude::*;
use model::HotkeyUnitDetailModel;
use tw_macro::assert_component;

/// The shared-hotkey detail pane. Composes the shared `DetailCard` surface, supplying its
/// body region — the dispatcher that shows the filled pane (the unit header over its
/// conflict cards) or the empty prompt.
#[component]
pub fn HotkeyUnitDetail(props: HotkeyUnitDetailModel) -> Element {
    let units = props.units;
    let body = HotkeyUnitDetailBodyView { units };
    rsx! {
        DetailCard::<HotkeyUnitDetailBodyView> { body }
    }
}

assert_component!(HotkeyUnitDetail);
