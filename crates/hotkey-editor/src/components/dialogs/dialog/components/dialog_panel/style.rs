use dioxus::prelude::*;

/// The panel's stylesheets: the desktop base plus the two viewport bands that
/// resize it for tablet and phone.
pub(super) const DIALOG_PANEL_STYLE_SHEETS: [Asset; 3] = [
    asset!("/src/components/dialogs/dialog/components/dialog_panel/styles/base.css"),
    asset!("/src/components/dialogs/dialog/components/dialog_panel/styles/tablet.css"),
    asset!("/src/components/dialogs/dialog/components/dialog_panel/styles/phone.css"),
];
