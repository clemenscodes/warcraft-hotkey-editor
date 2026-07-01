pub mod components;
mod props;
mod style;
use crate::assert_component;
use components::resolve_mini_icon::{ResolveMiniIcon, ResolveMiniIconProps};
use dioxus::prelude::*;
pub use props::ResolveMiniCellProps;
use style::CLASS;
assert_component!(ResolveMiniCell);
#[component]
pub fn ResolveMiniCell(props: ResolveMiniCellProps) -> Element {
    let has_placement = props.has_placement;
    let name = props.name;
    let icon = props
        .icon_url
        .map(|src| ResolveMiniIconProps { src, alt: name });
    rsx! {
        div {
            class: CLASS,
            "data-collision": has_placement,
            if let Some(icon) = icon {
                ResolveMiniIcon { ..icon }
            }
        }
    }
}
