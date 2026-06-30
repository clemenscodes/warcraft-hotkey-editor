use dioxus::prelude::*;

/// The backdrop's stylesheets: the desktop base plus the phone band that drops
/// the padding. The panel, body, and header own their own sizing.
pub(super) const DIALOG_STYLE_SHEETS: [Asset; 2] = [
    asset!("/src/components/dialogs/dialog/styles/base.css"),
    asset!("/src/components/dialogs/dialog/styles/phone.css"),
];
