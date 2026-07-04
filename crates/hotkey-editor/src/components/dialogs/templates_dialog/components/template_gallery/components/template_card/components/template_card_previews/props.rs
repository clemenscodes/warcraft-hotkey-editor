use super::super::super::TemplateCardProps;
use crate::components::grid_editors::grid_editor::components::grid_editor_tile::components::tile_face::{
    TileFaceKind, TileFaceProps,
};
use crate::components::grid_editors::grid_editor::components::headed_grid::HeadedGridProps;
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::GridProps;
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
pub(super) fn command_preview(resolved: &ResolvedTemplate) -> HeadedGridProps<TileFaceKind> {
    let tiles = preview_tiles(resolved.command_tiles());
    let kind = TileFaceKind;
    let grid = GridProps { kind, tiles };
    HeadedGridProps {
        heading: "Command card",
        grid,
    }
}

/// The research menu preview, captioned "Research menu".
pub(super) fn research_preview(resolved: &ResolvedTemplate) -> HeadedGridProps<TileFaceKind> {
    let tiles = preview_tiles(resolved.research_tiles());
    let kind = TileFaceKind;
    let grid = GridProps { kind, tiles };
    HeadedGridProps {
        heading: "Research menu",
        grid,
    }
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
