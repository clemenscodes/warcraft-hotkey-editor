mod props;
mod style;
use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, FramedIconProps, IconRadius,
};
use dioxus::prelude::*;
pub use props::FightIconProps;
use style::CLASS;
use tw_macro::assert_component;

/// The ability's square icon inside its carrier button: it owns its slot and lifts
/// to a gold glow when the enabled button `.group` is hovered. The shared
/// `FramedIcon` draws the bordered, rounded image.
#[component]
pub fn FightIcon(props: FightIconProps) -> Element {
    let Some(source) = props.src else {
        return rsx! {};
    };
    let src = Some(source);
    let alt = props.alt;
    let framed = FramedIconProps {
        src,
        alt,
        radius: IconRadius::Card,
        hover_glow: true,
        placeholder: false,
    };
    rsx! {
        div {
            class: CLASS,
            FramedIcon { ..framed }
        }
    }
}

assert_component!(FightIcon);
