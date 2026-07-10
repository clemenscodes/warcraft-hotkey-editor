use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile, ResolvedTemplate};

/// The previews row's input: the resolved layout both mini-grids read from.
#[derive(Props, Clone, PartialEq)]
pub struct TemplateCardPreviewsProps {
    pub resolved: ResolvedTemplate,
}

/// One captioned mini-grid: its heading and the twelve resolved domain tiles it draws.
/// The `PreviewGrid` adapts each tile to the shared `TileFace` painter at render time.
pub(super) struct TemplatePreview {
    pub(super) heading: &'static str,
    pub(super) tiles: [RenderedTile; COMMAND_GRID_TILE_COUNT],
}

/// The command card preview: a headed grid of the resolved command tiles — the same
/// tiles the editor draws, without any of its behaviour — captioned "Command card".
pub(super) fn command_preview(resolved: &ResolvedTemplate) -> TemplatePreview {
    let source = resolved.command_tiles();
    let tiles = preview_tiles(source);
    let heading = "Command card";
    TemplatePreview { heading, tiles }
}

/// The research menu preview, captioned "Research menu".
pub(super) fn research_preview(resolved: &ResolvedTemplate) -> TemplatePreview {
    let source = resolved.research_tiles();
    let tiles = preview_tiles(source);
    let heading = "Research menu";
    TemplatePreview { heading, tiles }
}

/// Collects a template's resolved tiles into the fixed-size grid array the preview
/// draws, so the mini-grid always renders exactly `COMMAND_GRID_TILE_COUNT` tiles.
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
