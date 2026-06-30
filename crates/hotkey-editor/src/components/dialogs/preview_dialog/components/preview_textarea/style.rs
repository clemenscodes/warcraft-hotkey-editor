use dioxus::prelude::*;

/// The textarea's stylesheets: the base plus the phone band that shrinks the
/// monospace text.
pub(super) const PREVIEW_TEXTAREA_STYLE_SHEETS: [Asset; 2] = [
    asset!("/src/components/dialogs/preview_dialog/components/preview_textarea/styles/base.css"),
    asset!("/src/components/dialogs/preview_dialog/components/preview_textarea/styles/phone.css"),
];
