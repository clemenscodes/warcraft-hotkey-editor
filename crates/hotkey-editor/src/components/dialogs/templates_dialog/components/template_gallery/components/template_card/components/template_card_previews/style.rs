use dioxus::prelude::*;

/// The previews row's stylesheets: the base row plus the mobile band that keeps
/// the two grids side by side without wrapping.
pub(super) const TEMPLATE_CARD_PREVIEWS_STYLE_SHEETS: [Asset; 2] = [
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/components/template_card_previews/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/templates_dialog/components/template_gallery/components/template_card/components/template_card_previews/styles/mobile.css"
    ),
];
