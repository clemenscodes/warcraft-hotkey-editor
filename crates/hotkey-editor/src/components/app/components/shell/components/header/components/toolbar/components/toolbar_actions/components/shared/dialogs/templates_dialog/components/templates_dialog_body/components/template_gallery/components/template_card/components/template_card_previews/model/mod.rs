use super::view::TemplateCardPreviewsView;
use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile, ResolvedTemplate};

#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardPreviewsModel {
    pub resolved: ResolvedTemplate,
}

pub(super) struct TemplatePreview {
    pub(super) heading: &'static str,
    pub(super) tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

pub(super) fn command_preview(resolved: &ResolvedTemplate) -> TemplatePreview {
    let source = resolved.command_tiles();
    let tiles = preview_tiles(source);
    let heading = "Command card";
    TemplatePreview { heading, tiles }
}

pub(super) fn research_preview(resolved: &ResolvedTemplate) -> TemplatePreview {
    let source = resolved.research_tiles();
    let tiles = preview_tiles(source);
    let heading = "Research menu";
    TemplatePreview { heading, tiles }
}

fn preview_tiles(source: &[RenderedTile]) -> [RenderedTile; COMMAND_GRID_TILE_COUNT] {
    let tile_list = source.to_vec();
    tile_list
        .try_into()
        .unwrap_or_else(|list: Vec<RenderedTile>| {
            panic!(
                "template preview grid must render exactly {COMMAND_GRID_TILE_COUNT} tiles, got {}",
                list.len(),
            )
        })
}

impl From<&TemplateCardPreviewsView> for TemplateCardPreviewsModel {
    fn from(view: &TemplateCardPreviewsView) -> Self {
        let TemplateCardPreviewsView { resolved } = view.clone();
        Self { resolved }
    }
}

impl ddd::Model for TemplateCardPreviewsModel {
    type View = TemplateCardPreviewsView;
}
