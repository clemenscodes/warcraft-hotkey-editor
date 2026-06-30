use dioxus::prelude::*;

/// The name heading's stylesheets: the base plus the mobile band that shrinks
/// and golds it.
pub(super) const TEMPLATE_CARD_NAME_STYLE_SHEETS: [Asset; 2] = [
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/components/template_card_text/components/template_card_name/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/components/template_card_text/components/template_card_name/styles/mobile.css"
    ),
];
