mod model;
mod view;

pub use view::PlaceholderIconView;
mod style;

use crate::components::app::components::shell::components::shared::framed_icon::components::shared::framed_icon_image::FramedIconImage;
use dioxus::prelude::*;
use model::PlaceholderIconModel;
use style::CLASS;
use tw_macro::assert_component;

/// The empty-placeholder look of a framed icon: the hairline-radius blue frame filled
/// with the panel surface. Presentational — the dispatcher builds its props and renders
/// it when the resolved look is `Placeholder`. Absent `source` draws the empty framed
/// square; a present `source` draws the covered image inside it.
#[component]
pub fn PlaceholderIcon(props: PlaceholderIconModel) -> Element {
    let Some(source) = props.source else {
        return rsx! {
            div { class: CLASS }
        };
    };
    let alt = props.alt;
    rsx! {
        div {
            class: CLASS,
            FramedIconImage { source, alt }
        }
    }
}

assert_component!(PlaceholderIcon);
