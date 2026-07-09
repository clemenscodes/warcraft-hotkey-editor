pub(crate) struct FocusedElementInfo {
    classes: String,
    is_inside_grid_panel: bool,
    is_inside_system_dialog: bool,
    is_inside_mode_tabs: bool,
}

impl FocusedElementInfo {
    pub(crate) fn current() -> Option<Self> {
        let active_element = web_sys::window()?.document()?.active_element()?;
        let classes = active_element.class_name();
        let is_inside_grid_panel = classes.contains("grid-editor-tile")
            || classes.contains("override-key")
            || classes.contains("tile-override-tier-button");
        let is_inside_system_dialog = active_element
            .closest(".dialog")
            .ok()
            .flatten()
            .and_then(|dialog| dialog.query_selector(".key-capture").ok().flatten())
            .is_some();
        let is_inside_mode_tabs = active_element
            .closest(".mode-tabs")
            .ok()
            .flatten()
            .is_some();
        let info = Self {
            classes,
            is_inside_grid_panel,
            is_inside_system_dialog,
            is_inside_mode_tabs,
        };
        Some(info)
    }

    pub(crate) fn classes(&self) -> &str {
        &self.classes
    }

    pub(crate) fn is_inside_grid_panel(&self) -> bool {
        self.is_inside_grid_panel
    }

    pub(crate) fn is_inside_system_dialog(&self) -> bool {
        self.is_inside_system_dialog
    }

    pub(crate) fn is_inside_mode_tabs(&self) -> bool {
        self.is_inside_mode_tabs
    }
}

/// The app's focus-movement seam. The generic DOM traversal lives in
/// `browser_kit::dom::RovingFocus`; these methods bind the app's container and focusable
/// selectors.
pub(crate) struct FocusNavigation;

impl FocusNavigation {
    pub(crate) fn first_matching(selectors: &[&str]) -> bool {
        browser_kit::dom::RovingFocus::first_matching(selectors)
    }

    pub(crate) fn cycle_inside_unit_detail(reverse: bool) {
        browser_kit::dom::RovingFocus::cycle(
            ".unit-detail",
            ".grid-editor-tile, .override-key, .tile-override-tier-button",
            reverse,
        );
    }

    pub(crate) fn cycle_inside_system_dialog(reverse: bool) {
        browser_kit::dom::RovingFocus::cycle(".dialog", ".dialog-close, .key-capture", reverse);
    }
}
