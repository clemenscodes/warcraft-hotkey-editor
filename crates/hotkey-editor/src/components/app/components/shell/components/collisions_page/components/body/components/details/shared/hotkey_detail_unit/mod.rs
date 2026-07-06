pub mod components;
mod props;
mod style;
use components::hotkey_detail_unit_icon::{HotkeyDetailUnitIcon, HotkeyDetailUnitIconProps};
use dioxus::prelude::*;
pub use props::HotkeyDetailUnitProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HotkeyDetailUnit);
/// The clickable unit portrait in the detail header (deep-links into the editor).
#[component]
pub fn HotkeyDetailUnit(props: HotkeyDetailUnitProps) -> Element {
    let onclick = props.onclick;
    let name = props.name;
    let icon = HotkeyDetailUnitIconProps {
        src: props.icon_url,
        alt: name,
    };
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            HotkeyDetailUnitIcon { ..icon }
        }
    }
}
