use dioxus::prelude::*;

/// The grid editor's own stylesheet: the flex column that stacks the heading
/// above the grid. The per-viewport sizing lives in the grid and heading
/// components, so this is a single sheet.
pub(super) const GRID_EDITOR_STYLES: Asset =
    asset!("/src/components/grid_editors/grid_editor/styles/base.css");
