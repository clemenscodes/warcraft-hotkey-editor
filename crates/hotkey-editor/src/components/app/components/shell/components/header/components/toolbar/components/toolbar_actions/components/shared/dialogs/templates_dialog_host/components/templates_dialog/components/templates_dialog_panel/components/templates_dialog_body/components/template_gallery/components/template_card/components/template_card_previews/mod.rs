pub mod components;
mod props;
mod style;

use components::preview_headed_grid::PreviewHeadedGrid;
use dioxus::prelude::*;
pub use props::TemplateCardPreviewsProps;
use props::{command_preview, research_preview};
use style::CLASS;
use tw_macro::assert_component;

/// The row of two layout previews under a card: the command card and the
/// research menu, each a read-only `PreviewHeadedGrid` drawing the template.
/// Owns `.template-card-previews`.
#[component]
pub fn TemplateCardPreviews(props: TemplateCardPreviewsProps) -> Element {
    let command = command_preview(&props.resolved);
    let research = research_preview(&props.resolved);
    rsx! {
        div { class: CLASS,
            PreviewHeadedGrid { ..command }
            PreviewHeadedGrid { ..research }
        }
    }
}

assert_component!(TemplateCardPreviews);
