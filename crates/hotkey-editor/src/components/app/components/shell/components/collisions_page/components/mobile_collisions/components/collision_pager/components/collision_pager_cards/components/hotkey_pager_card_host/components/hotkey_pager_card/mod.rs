mod model;
mod view;

pub use view::HotkeyPagerCardView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::hotkey_unit_detail::components::hotkey_unit_detail_body::components::filled_hotkey_unit_detail::FilledHotkeyUnitDetail;
use dioxus::prelude::*;
use model::HotkeyPagerCardModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HotkeyPagerCard(props: HotkeyPagerCardModel) -> Element {
    let unit = props.unit;
    rsx! {
        div {
            class: CLASS,
            FilledHotkeyUnitDetail {
                unit_view: unit,
            }
        }
    }
}

assert_component!(HotkeyPagerCard);
