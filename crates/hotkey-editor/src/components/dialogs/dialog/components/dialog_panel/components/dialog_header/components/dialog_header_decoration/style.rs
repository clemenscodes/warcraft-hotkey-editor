use dioxus::prelude::*;

/// The decoration's stylesheets: the desktop base plus the two viewport bands
/// that shrink its width on smaller screens.
pub(super) const DIALOG_HEADER_DECORATION_STYLE_SHEETS: [Asset; 3] = [
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/components/dialog_header_decoration/styles/base.css"
    ),
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/components/dialog_header_decoration/styles/tablet.css"
    ),
    asset!(
        "/src/components/dialogs/dialog/components/dialog_panel/components/dialog_header/components/dialog_header_decoration/styles/phone.css"
    ),
];
