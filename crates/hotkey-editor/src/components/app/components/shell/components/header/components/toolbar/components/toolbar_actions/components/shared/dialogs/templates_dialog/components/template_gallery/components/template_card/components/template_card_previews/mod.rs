mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::headed_grid::HeadedGrid;
use dioxus::prelude::*;
pub use props::TemplateCardPreviewsProps;
use props::{command_preview, research_preview};
use style::CLASS;
assert_component!(TemplateCardPreviews);

/// The row of two layout previews under a card: the command card and the
/// research menu, each a reused `HeadedGrid` drawing the template read-only.
/// Owns `.template-card-previews`.
#[component]
pub fn TemplateCardPreviews(props: TemplateCardPreviewsProps) -> Element {
    let command = command_preview(&props.resolved);
    let research = research_preview(&props.resolved);
    rsx! {
        div { class: CLASS,
            HeadedGrid { ..command }
            HeadedGrid { ..research }
        }
    }
}
