use dioxus::prelude::*;

/// The captioned grid's stylesheet: the column that stacks the heading above the
/// grid. One sheet; the heading and grid own their own sizing.
pub(super) const HEADED_GRID_STYLE_SHEETS: Asset =
    asset!("/src/components/grid_editors/grid_editor/components/headed_grid/styles/base.css");
