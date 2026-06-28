use dioxus::prelude::*;

/// Per-viewport stylesheets for the heading. `base.css` holds the look and the
/// fallback size; each remaining file scopes one viewport band's size behind its
/// own `@media` query, so the heading grows in step with the tiles. See
/// `docs/CSS_VIEWPORTS.md`.
pub(super) const GRID_HEADING_STYLE_SHEETS: [Asset; 8] = [
    asset!("/src/components/grid_editors/grid_editor/components/grid_heading/styles/base.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid_heading/styles/phone.css"),
    asset!(
        "/src/components/grid_editors/grid_editor/components/grid_heading/styles/large_phone.css"
    ),
    asset!("/src/components/grid_editors/grid_editor/components/grid_heading/styles/tablet.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid_heading/styles/desktop.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid_heading/styles/full_hd.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid_heading/styles/wide.css"),
    asset!("/src/components/grid_editors/grid_editor/components/grid_heading/styles/four_k.css"),
];
