mod props;
mod style;
use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, FramedIconProps, IconRadius,
};
use dioxus::prelude::*;
pub use props::ConflictDetailUnitIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictDetailUnitIcon);

/// The unit portrait in the detail header: it owns its slot and the shared
/// `FramedIcon` draws the bordered, rounded image.
#[component]
pub fn ConflictDetailUnitIcon(props: ConflictDetailUnitIconProps) -> Element {
    let Some(source) = props.src else {
        return rsx! {};
    };
    let src = Some(source);
    let alt = props.alt;
    let framed = FramedIconProps {
        src,
        alt,
        radius: IconRadius::Control,
        hover_glow: false,
        placeholder: false,
    };
    rsx! {
        div {
            class: CLASS,
            FramedIcon { ..framed }
        }
    }
}
