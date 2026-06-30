use dioxus::prelude::*;

/// The gallery's stylesheets: the two-column desktop base plus the mobile band
/// that collapses to a single column.
pub(super) const TEMPLATE_GALLERY_STYLE_SHEETS: [Asset; 2] = [
    asset!("/src/components/dialogs/templates_dialog/components/template_gallery/styles/base.css"),
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/styles/mobile.css"
    ),
];
