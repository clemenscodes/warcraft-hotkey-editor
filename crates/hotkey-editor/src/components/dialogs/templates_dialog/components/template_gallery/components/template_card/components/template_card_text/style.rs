use dioxus::prelude::*;

/// The text block's stylesheets: the base column plus the mobile band that
/// tightens its gap.
pub(super) const TEMPLATE_CARD_TEXT_STYLE_SHEETS: [Asset; 2] = [
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/components/template_card_text/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/components/template_card_text/styles/mobile.css"
    ),
];
