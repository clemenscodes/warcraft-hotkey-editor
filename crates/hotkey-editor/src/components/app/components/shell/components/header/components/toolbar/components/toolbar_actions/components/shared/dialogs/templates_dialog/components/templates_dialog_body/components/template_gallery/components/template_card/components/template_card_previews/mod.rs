pub mod components;
mod model;
mod view;

pub use view::TemplateCardPreviewsView;
mod style;

use components::preview_headed_grid::PreviewHeadedGrid;
use dioxus::prelude::*;
use model::TemplateCardPreviewsModel;
use model::{TemplatePreview, command_preview, research_preview};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TemplateCardPreviews(props: TemplateCardPreviewsModel) -> Element {
    let resolved = props.resolved;
    let TemplatePreview {
        heading: command_heading,
        tiles: command_tiles,
    } = command_preview(&resolved);
    let TemplatePreview {
        heading: research_heading,
        tiles: research_tiles,
    } = research_preview(&resolved);
    rsx! {
        div {
            class: CLASS,
            PreviewHeadedGrid {
                heading: command_heading,
                tiles: command_tiles,
            }
            PreviewHeadedGrid {
                heading: research_heading,
                tiles: research_tiles,
            }
        }
    }
}

assert_component!(TemplateCardPreviews);
