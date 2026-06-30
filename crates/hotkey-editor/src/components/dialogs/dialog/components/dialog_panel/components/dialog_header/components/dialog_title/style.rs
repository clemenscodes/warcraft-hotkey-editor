use dioxus::prelude::*;

/// The title's stylesheets: the desktop base plus the two viewport bands that
/// shrink and truncate it so a long heading never wraps or overflows.
pub(super) const DIALOG_TITLE_STYLE_SHEETS: [Asset; 3] = [
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/components/dialog_title/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/components/dialog_title/styles/tablet.css"
    ),
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/components/dialog_title/styles/phone.css"
    ),
];
