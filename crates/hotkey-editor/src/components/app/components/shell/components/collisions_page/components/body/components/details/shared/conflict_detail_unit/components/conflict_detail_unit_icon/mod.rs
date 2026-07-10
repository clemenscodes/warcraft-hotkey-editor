mod props;
mod view;

pub use view::ConflictDetailUnitIconView;
mod style;
use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, IconRadius,
};
use dioxus::prelude::*;
use props::ConflictDetailUnitIconProps;
use style::CLASS;
use tw_macro::assert_component;

/// The unit portrait in the detail header: it owns its slot and the shared
/// `FramedIcon` draws the bordered, rounded image.
#[component]
pub fn ConflictDetailUnitIcon(props: ConflictDetailUnitIconProps) -> Element {
    let Some(source) = props.src else {
        return rsx! {};
    };
    let src = Some(source);
    let alt = props.alt;
    let radius = IconRadius::Control;
    rsx! {
        div {
            class: CLASS,
            FramedIcon {
                src,
                alt,
                radius,
                hover_glow: false,
                placeholder: false,
            }
        }
    }
}

assert_component!(ConflictDetailUnitIcon);
