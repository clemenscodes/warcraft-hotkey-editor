mod props;
mod style;

use dioxus::prelude::*;

use props::DialogHeaderDecorationPresentation;
use style::DIALOG_HEADER_DECORATION_STYLE_SHEETS;

pub use props::DialogHeaderDecorationProps;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

/// A gold flourish flanking the dialog title. Owns `.dialog-header-decoration`;
/// the trailing instance carries the flipped modifier.
#[component]
pub fn DialogHeaderDecoration(props: DialogHeaderDecorationProps) -> Element {
    let DialogHeaderDecorationPresentation { class } =
        DialogHeaderDecorationPresentation::from(&props);
    rsx! {
        for href in DIALOG_HEADER_DECORATION_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        img {
            class,
            src: HEADER_GOLD_DECORATION,
            alt: "",
            aria_hidden: "true",
        }
    }
}
