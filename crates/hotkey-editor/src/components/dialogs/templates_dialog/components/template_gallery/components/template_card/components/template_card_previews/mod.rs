mod props;
mod style;

use dioxus::prelude::*;

use crate::components::grid_editors::grid_editor::components::headed_grid::HeadedGrid;
use props::{command_preview, research_preview};
use style::TEMPLATE_CARD_PREVIEWS_STYLE_SHEETS;

pub use props::TemplateCardPreviewsProps;

/// The row of two layout previews under a card: the command card and the
/// research menu, each a reused `HeadedGrid` drawing the template read-only.
/// Owns `.template-card-previews`.
#[component]
pub fn TemplateCardPreviews(props: TemplateCardPreviewsProps) -> Element {
    let command = command_preview(&props.resolved);
    let research = research_preview(&props.resolved);
    rsx! {
        for href in TEMPLATE_CARD_PREVIEWS_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class: "template-card-previews",
            HeadedGrid { ..command }
            HeadedGrid { ..research }
        }
    }
}
