use dioxus::prelude::*;

/// The body's stylesheets: the base scroll region plus the phone band that
/// tightens the gutter.
pub(super) const DIALOG_BODY_STYLE_SHEETS: [Asset; 2] = [
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_body/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_body/styles/phone.css"
    ),
];
