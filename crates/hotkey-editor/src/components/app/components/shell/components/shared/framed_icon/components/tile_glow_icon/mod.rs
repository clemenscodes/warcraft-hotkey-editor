mod props;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::components::shared::framed_icon_image::{
    FramedIconImage, FramedIconImageProps,
};
use dioxus::prelude::*;
pub use props::TileGlowIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(TileGlowIcon);

/// The glowing tile look of a framed icon: the tile-radius blue frame that lifts to a
/// gold hover glow. Presentational — the dispatcher builds its props and renders it
/// when the resolved look is `TileGlow`. Absent `source` draws the empty framed square.
#[component]
pub fn TileGlowIcon(props: TileGlowIconProps) -> Element {
    let Some(source) = props.source else {
        return rsx! {
            div { class: CLASS }
        };
    };
    let alt = props.alt;
    let image = FramedIconImageProps { source, alt };
    rsx! {
        div {
            class: CLASS,
            FramedIconImage { ..image }
        }
    }
}
