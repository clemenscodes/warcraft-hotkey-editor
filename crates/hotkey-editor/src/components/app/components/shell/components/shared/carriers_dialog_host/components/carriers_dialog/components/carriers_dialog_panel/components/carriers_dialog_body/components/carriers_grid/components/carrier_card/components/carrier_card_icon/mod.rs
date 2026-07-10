mod props;
mod view;

pub use view::CarrierCardIconView;
mod style;
use crate::components::app::components::shell::components::shared::framed_icon::{
    FramedIcon, IconRadius,
};
use dioxus::prelude::*;
use props::CarrierCardIconProps;
use style::CLASS;
use tw_macro::assert_component;

/// A carrier unit's square icon: it owns its fixed slot and the shared `FramedIcon`
/// draws the bordered, rounded image.
#[component]
pub fn CarrierCardIcon(props: CarrierCardIconProps) -> Element {
    let Some(source) = props.src else {
        return rsx! {};
    };
    let src = Some(source);
    let alt = props.alt;
    rsx! {
        div {
            class: CLASS,
            FramedIcon {
                src,
                alt,
                radius: IconRadius::Tile,
                hover_glow: false,
                placeholder: false,
            }
        }
    }
}

assert_component!(CarrierCardIcon);
