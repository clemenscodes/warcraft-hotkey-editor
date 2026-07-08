mod props;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::components::shared::framed_icon_image::{
    FramedIconImage, FramedIconImageProps,
};
use dioxus::prelude::*;
pub use props::PlaceholderIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PlaceholderIcon);

/// The empty-placeholder look of a framed icon: the hairline-radius blue frame filled
/// with the panel surface. Presentational — the dispatcher builds its props and renders
/// it when the resolved look is `Placeholder`. Absent `source` draws the empty framed
/// square; a present `source` draws the covered image inside it.
#[component]
pub fn PlaceholderIcon(props: PlaceholderIconProps) -> Element {
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
