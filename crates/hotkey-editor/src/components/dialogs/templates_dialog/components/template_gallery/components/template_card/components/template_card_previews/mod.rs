mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::grid_editors::grid_editor::components::headed_grid::HeadedGrid;
use props::{command_preview, research_preview};
use style::CLASS;

pub use props::TemplateCardPreviewsProps;

assert_component!(TemplateCardPreviews);

/// The row of two layout previews under a card: the command card and the
/// research menu, each a reused `HeadedGrid` drawing the template read-only.
/// Owns `.template-card-previews`.
#[component]
pub fn TemplateCardPreviews(props: TemplateCardPreviewsProps) -> Element {
    let command = command_preview(&props.resolved);
    let research = research_preview(&props.resolved);
    rsx! {
        div {
            class: CLASS,
            HeadedGrid { ..command }
            HeadedGrid { ..research }
        }
    }
}
