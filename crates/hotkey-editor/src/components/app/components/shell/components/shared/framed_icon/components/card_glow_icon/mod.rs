mod props;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::components::shared::framed_icon_image::{
    FramedIconImage, FramedIconImageProps,
};
use dioxus::prelude::*;
pub use props::CardGlowIconProps;
use style::CLASS;
use tw_macro::assert_component;

/// The glowing card look of a framed icon: the card-radius blue frame that lifts to a
/// gold hover glow. Presentational — the dispatcher builds its props and renders it
/// when the resolved look is `CardGlow`. Absent `source` draws the empty framed square.
#[component]
pub fn CardGlowIcon(props: CardGlowIconProps) -> Element {
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

assert_component!(CardGlowIcon);
