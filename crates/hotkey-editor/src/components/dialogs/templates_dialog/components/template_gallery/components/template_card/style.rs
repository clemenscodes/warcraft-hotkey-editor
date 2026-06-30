use dioxus::prelude::*;

/// The card's stylesheets: the desktop base with hover and focus states plus the
/// mobile band that tightens its padding.
pub(super) const TEMPLATE_CARD_STYLE_SHEETS: [Asset; 2] = [
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/styles/mobile.css"
    ),
];
