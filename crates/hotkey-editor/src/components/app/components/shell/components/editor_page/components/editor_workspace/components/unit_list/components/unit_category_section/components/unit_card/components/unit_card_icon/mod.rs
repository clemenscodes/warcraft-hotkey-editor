pub mod components;
mod props;
mod style;

use components::unit_card_portrait::{UnitCardPortrait, UnitCardPortraitProps};
use dioxus::prelude::*;
pub use props::UnitCardIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCardIcon);

/// The framed portrait slot of a unit card: it owns the per-band icon size, and the
/// portrait image fills it. Renders as an empty framed square when the unit has no
/// icon.
#[component]
pub fn UnitCardIcon(props: UnitCardIconProps) -> Element {
    let icon_path = props.icon_path;
    let display_name = props.display_name;
    let icon_url = icon_path.map(|url| url.to_string());
    let portrait = icon_url.map(|source| {
        let portrait_props = UnitCardPortraitProps {
            source,
            display_name,
        };
        rsx! {
            UnitCardPortrait { ..portrait_props }
        }
    });
    rsx! {
        div { class: CLASS, {portrait} }
    }
}
