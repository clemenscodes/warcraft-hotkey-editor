mod style;

use dioxus::prelude::*;

use style::DIALOG_TITLE_STYLE_SHEETS;

/// The dialog's heading text. Owns `.dialog-title`. A leaf: the header passes the
/// title as children.
#[derive(Props, Clone, PartialEq)]
pub struct DialogTitleProps {
    pub children: Element,
}

#[component]
pub fn DialogTitle(props: DialogTitleProps) -> Element {
    let title = props.children.clone();
    rsx! {
        for href in DIALOG_TITLE_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        h2 {
            class: "dialog-title",
            {title}
        }
    }
}
