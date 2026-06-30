use dioxus::prelude::*;

/// The header bar's stylesheets: the desktop base plus the two viewport bands
/// that tighten the gutters on smaller screens.
pub(super) const DIALOG_HEADER_STYLE_SHEETS: [Asset; 3] = [
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/styles/tablet.css"
    ),
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/styles/phone.css"
    ),
];
