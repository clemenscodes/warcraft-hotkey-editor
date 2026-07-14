mod model;
mod presentation;
mod view;

pub use view::UnitCardIconView;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::FramedIcon;
use dioxus::prelude::*;
use model::UnitCardIconModel;
use presentation::UnitCardIconPresentation;
use style::CLASS;
use tw_macro::assert_component;

/// The framed portrait slot of a unit card: it owns the per-band icon size, and the
/// shared `FramedIcon` draws the bordered, rounded image — or the empty framed square
/// when the unit has no icon.
#[component]
pub fn UnitCardIcon(props: UnitCardIconModel) -> Element {
    let UnitCardIconPresentation {
        src,
        alt,
        radius,
        hover_glow,
        placeholder,
    } = UnitCardIconPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            FramedIcon {
                src,
                alt,
                radius,
                hover_glow,
                placeholder,
            }
        }
    }
}

assert_component!(UnitCardIcon);
