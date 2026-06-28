use dioxus::prelude::*;

/// The section wrapper's own stylesheet: the flex column that stacks the heading
/// above the grid. The per-viewport sizing lives in the grid and heading
/// components, so this is a single sheet.
pub(super) const COMMAND_GRID_SECTION_STYLES: Asset =
    asset!("/src/components/command_grid/components/section/styles/base.css");
