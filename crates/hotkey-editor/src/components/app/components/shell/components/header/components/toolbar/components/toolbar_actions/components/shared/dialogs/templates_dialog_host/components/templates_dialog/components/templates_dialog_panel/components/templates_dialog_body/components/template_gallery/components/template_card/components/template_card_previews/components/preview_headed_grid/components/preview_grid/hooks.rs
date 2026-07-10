use super::logic::PreviewTile;
use super::props::PreviewGridProps;

/// Adapts the preview grid's resolved domain tiles into the painted values each
/// `TileFace` places, so the body stays a flat loop over already-shaped data.
pub(super) fn use_preview_grid(props: &PreviewGridProps) -> Vec<PreviewTile> {
    props.tiles.iter().map(PreviewTile::from).collect()
}
