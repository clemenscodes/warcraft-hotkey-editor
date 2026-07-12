mod model;
mod view;

pub use view::UnitCardIconView;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, IconRadius,
};
use dioxus::prelude::*;
use model::UnitCardIconModel;
use style::CLASS;
use tw_macro::assert_component;

/// The framed portrait slot of a unit card: it owns the per-band icon size, and the
/// shared `FramedIcon` draws the bordered, rounded image — or the empty framed square
/// when the unit has no icon.
#[component]
pub fn UnitCardIcon(props: UnitCardIconModel) -> Element {
    let icon_path = props.icon_path;
    let display_name = props.display_name;
    let src = icon_path.map(|url| url.to_string());
    let radius = IconRadius::Hairline;
    let hover_glow = false;
    let placeholder = true;
    rsx! {
        div {
            class: CLASS,
            FramedIcon {
                src,
                alt: display_name,
                radius,
                hover_glow,
                placeholder,
            }
        }
    }
}

assert_component!(UnitCardIcon);
