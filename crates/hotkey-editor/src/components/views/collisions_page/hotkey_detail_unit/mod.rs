pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::hotkey_detail_unit_icon::{HotkeyDetailUnitIcon, HotkeyDetailUnitIconProps};
use dioxus::prelude::*;
pub use props::HotkeyDetailUnitProps;
use style::CLASS;
assert_component!(HotkeyDetailUnit);
/// The clickable unit portrait in the detail header (deep-links into the editor).
#[component]
pub fn HotkeyDetailUnit(props: HotkeyDetailUnitProps) -> Element {
    let onclick = props.onclick;
    let name = props.name;
    let icon = props
        .icon_url
        .map(|src| HotkeyDetailUnitIconProps { src, alt: name });
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            if let Some(icon) = icon {
                HotkeyDetailUnitIcon { ..icon }
            }
        }
    }
}
