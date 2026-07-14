pub mod components;
mod model;
mod view;

pub use view::HotkeyUnitRowIconHostView;
mod style;

use components::hotkey_unit_row_icon::HotkeyUnitRowIcon;
use dioxus::prelude::*;
use model::HotkeyUnitRowIconHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HotkeyUnitRowIconHost(props: HotkeyUnitRowIconHostModel) -> Element {
    let Some(source) = props.icon_url else {
        return rsx! {};
    };
    let icon_url = Some(source);
    let alt = props.alt;
    rsx! {
        div {
            class: CLASS,
            HotkeyUnitRowIcon {
                icon_url,
                alt,
            }
        }
    }
}

assert_component!(HotkeyUnitRowIconHost);
