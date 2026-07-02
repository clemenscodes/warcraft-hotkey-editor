pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::mini_icon::{MiniIcon, MiniIconProps};
use dioxus::prelude::*;
pub use props::MiniCellProps;
use style::CLASS;
assert_component!(MiniCell);
#[component]
pub fn MiniCell(props: MiniCellProps) -> Element {
    let has_placement = props.has_placement;
    let name = props.name;
    let icon = MiniIconProps {
        src: props.icon_url,
        alt: name,
    };
    rsx! {
        div {
            class: CLASS,
            "data-collision": has_placement,
            MiniIcon { ..icon }
        }
    }
}
