pub mod components;
mod model;
mod view;

pub use view::HotkeyPagerCardHostView;
mod style;

use components::hotkey_pager_card::HotkeyPagerCard;
use dioxus::prelude::*;
use model::HotkeyPagerCardHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HotkeyPagerCardHost(props: HotkeyPagerCardHostModel) -> Element {
    let unit = props.unit;
    rsx! {
        div {
            class: CLASS,
            HotkeyPagerCard {
                unit,
            }
        }
    }
}

assert_component!(HotkeyPagerCardHost);
