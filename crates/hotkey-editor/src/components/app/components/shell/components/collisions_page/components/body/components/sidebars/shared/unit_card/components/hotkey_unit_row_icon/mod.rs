pub mod components;
mod props;
mod style;

use components::hotkey_unit_row_image::{HotkeyUnitRowImage, HotkeyUnitRowImageProps};
use dioxus::prelude::*;
pub use props::HotkeyUnitRowIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HotkeyUnitRowIcon);
/// A unit's portrait on a collision card. A guarded host: renders nothing when
/// the unit has no icon; otherwise a framed slot whose image fills it.
#[component]
pub fn HotkeyUnitRowIcon(props: HotkeyUnitRowIconProps) -> Element {
    let Some(source) = props.icon_url else {
        return rsx! {};
    };
    let alt = props.alt;
    let image_props = HotkeyUnitRowImageProps { source, alt };
    rsx! {
        div { class: CLASS,
            HotkeyUnitRowImage { ..image_props }
        }
    }
}
