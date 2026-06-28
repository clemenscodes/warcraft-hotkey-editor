use dioxus::prelude::*;

/// Per-viewport stylesheets for the heading. `base.css` holds the look and the
/// fallback size; each remaining file scopes one viewport band's size behind its
/// own `@media` query, so the heading grows in step with the tiles. See
/// `docs/CSS_VIEWPORTS.md`.
pub(super) const COMMAND_GRID_HEADING_STYLE_SHEETS: [Asset; 8] = [
    asset!("/src/components/command_grid/components/heading/styles/base.css"),
    asset!("/src/components/command_grid/components/heading/styles/phone.css"),
    asset!("/src/components/command_grid/components/heading/styles/large_phone.css"),
    asset!("/src/components/command_grid/components/heading/styles/tablet.css"),
    asset!("/src/components/command_grid/components/heading/styles/desktop.css"),
    asset!("/src/components/command_grid/components/heading/styles/full_hd.css"),
    asset!("/src/components/command_grid/components/heading/styles/wide.css"),
    asset!("/src/components/command_grid/components/heading/styles/four_k.css"),
];
