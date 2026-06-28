use dioxus::prelude::*;

/// Per-viewport stylesheets for the command grid. `base.css` holds the grid
/// track and the fallback tile cap; each remaining file scopes one viewport
/// band's cap behind its own `@media` query. See `docs/CSS_VIEWPORTS.md`.
pub(super) const GRID_STYLE_SHEETS: [Asset; 8] = [
    asset!("/src/components/grid_editors/grid_editor/components/grid/styles/base.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid/styles/phone.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid/styles/large_phone.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid/styles/tablet.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid/styles/desktop.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid/styles/full_hd.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid/styles/wide.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid/styles/four_k.css"),
];
