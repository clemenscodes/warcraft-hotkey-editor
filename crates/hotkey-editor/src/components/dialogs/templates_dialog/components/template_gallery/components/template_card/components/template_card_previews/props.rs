use dioxus::prelude::*;
use warcraft_keybinds::{COMMAND_GRID_TILE_COUNT, RenderedTile, ResolvedTemplate};
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::components::grid_tile::GridTileProps;
use crate::components::grid_editors::grid_editor::components::headed_grid::HeadedGridProps;
use super::super::super::TemplateCardProps;

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

/// The command card preview: the reused headed grid captioned "Command card",
/// drawing the template's command tiles read-only.
pub(super) fn command_preview(resolved: &ResolvedTemplate) -> HeadedGridProps {
    let tiles = preview_tiles(resolved.command_tiles());
    HeadedGridProps {
        heading: "Command card",
        tiles,
    }
}

/// The research menu preview, captioned "Research menu".
pub(super) fn research_preview(resolved: &ResolvedTemplate) -> HeadedGridProps {
    let tiles = preview_tiles(resolved.research_tiles());
    HeadedGridProps {
        heading: "Research menu",
        tiles,
    }
}

/// Adapts a template's resolved tiles into read-only `GridTileProps`, no handlers,
/// so the preview draws the same tiles the editor does without its behavior.
fn preview_tiles(source: &[RenderedTile]) -> [GridTileProps; COMMAND_GRID_TILE_COUNT] {
    let tile_list: Vec<GridTileProps> = source.iter().map(GridTileProps::from).collect();
    tile_list
        .try_into()
        .unwrap_or_else(|list: Vec<GridTileProps>| {
            panic!(
                "template preview grid must render exactly {COMMAND_GRID_TILE_COUNT} tiles, got {}",
                list.len(),
            )
        })
}
