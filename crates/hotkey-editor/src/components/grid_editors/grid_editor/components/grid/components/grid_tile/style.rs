use dioxus::prelude::*;

/// Per-viewport stylesheets for the tile wrapper. `base.css` holds the structure
/// and the fallback tile cap; each remaining file scopes one viewport band's cap
/// behind its own `@media` query. See `docs/CSS_VIEWPORTS.md`.
pub(super) const GRID_TILE_STYLE_SHEETS: [Asset; 8] = [
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid/components/grid_tile/styles/base.css"
    ),
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid/components/grid_tile/styles/phone.css"
    ),
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid/components/grid_tile/styles/large_phone.css"
    ),
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid/components/grid_tile/styles/tablet.css"
    ),
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid/components/grid_tile/styles/desktop.css"
    ),
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid/components/grid_tile/styles/full_hd.css"
    ),
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid/components/grid_tile/styles/wide.css"
    ),
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid/components/grid_tile/styles/four_k.css"
    ),
];
