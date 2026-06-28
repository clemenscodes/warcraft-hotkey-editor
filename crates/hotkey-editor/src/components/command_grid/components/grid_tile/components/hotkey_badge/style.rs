use dioxus::prelude::*;

/// The badge sizes itself with container-relative units, so a single stylesheet
/// covers every viewport. See `docs/CSS_VIEWPORTS.md`.
pub(super) const HOTKEY_BADGE_STYLES: Asset = asset!(
    "/src/components/command_grid/components/grid_tile/components/hotkey_badge/styles/base.css"
);
