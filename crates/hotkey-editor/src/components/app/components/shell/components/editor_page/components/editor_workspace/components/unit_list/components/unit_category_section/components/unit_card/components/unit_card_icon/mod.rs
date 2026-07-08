mod props;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, FramedIconProps, IconRadius,
};
use dioxus::prelude::*;
pub use props::UnitCardIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCardIcon);

/// The framed portrait slot of a unit card: it owns the per-band icon size, and the
/// shared `FramedIcon` draws the bordered, rounded image — or the empty framed square
/// when the unit has no icon.
#[component]
pub fn UnitCardIcon(props: UnitCardIconProps) -> Element {
    let icon_path = props.icon_path;
    let display_name = props.display_name;
    let src = icon_path.map(|url| url.to_string());
    let framed = FramedIconProps {
        src,
        alt: display_name,
        radius: IconRadius::Hairline,
        hover_glow: false,
        placeholder: true,
    };
    rsx! {
        div {
            class: CLASS,
            FramedIcon { ..framed }
        }
    }
}
