use dioxus::prelude::*;

/// The description line's stylesheets: the base plus the mobile band that
/// shrinks it.
pub(super) const TEMPLATE_CARD_DESCRIPTION_STYLE_SHEETS: [Asset; 2] = [
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/components/template_card_text/components/template_card_description/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/components/template_card_text/components/template_card_description/styles/mobile.css"
    ),
];
