use dioxus::prelude::*;

/// The follower sizes itself with container-relative units, so a single
/// stylesheet covers every viewport. See `docs/CSS_VIEWPORTS.md`.
pub(super) const DRAG_FOLLOWER_STYLES: Asset = asset!(
    "/src/components/grid_editors/grid_editor/components/grid/components/drag_follower_overlay/styles/base.css"
);
