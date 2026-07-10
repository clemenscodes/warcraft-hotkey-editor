use super::super::super::TemplateCardProps;
use super::components::preview_headed_grid::PreviewHeadedGridProps;
use crate::components::app::components::shell::components::shared::tile_face::TileFaceProps;
use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile, ResolvedTemplate};

/// The previews row's input: the resolved layout both mini-grids read from.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardPreviewsProps {
    pub resolved: ResolvedTemplate,
}

impl From<&TemplateCardProps> for TemplateCardPreviewsProps {
    fn from(props: &TemplateCardProps) -> Self {
        let resolved = props.resolved.clone();
        Self { resolved }
    }
}

/// The command card preview: a headed grid of read-only `TileFace` painters — the same
/// tiles the editor draws, without any of its behaviour — captioned "Command card".
pub(super) fn command_preview(resolved: &ResolvedTemplate) -> PreviewHeadedGridProps {
    let tiles = preview_tiles(resolved.command_tiles());
    let heading = "Command card";
    PreviewHeadedGridProps { heading, tiles }
}

/// The research menu preview, captioned "Research menu".
pub(super) fn research_preview(resolved: &ResolvedTemplate) -> PreviewHeadedGridProps {
    let tiles = preview_tiles(resolved.research_tiles());
    let heading = "Research menu";
    PreviewHeadedGridProps { heading, tiles }
}

/// Adapts a template's resolved tiles into read-only `TileFaceProps` — pure paint, no
/// handlers — so the preview draws the same tiles the editor does without its behaviour.
fn preview_tiles(source: &[RenderedTile]) -> [TileFaceProps; COMMAND_GRID_TILE_COUNT] {
    let tile_list: Vec<TileFaceProps> = source.iter().map(TileFaceProps::from).collect();
    tile_list
        .try_into()
        .unwrap_or_else(|list: Vec<TileFaceProps>| {
            panic!(
                "template preview grid must render exactly {COMMAND_GRID_TILE_COUNT} tiles, got {}",
                list.len(),
            )
        })
}
