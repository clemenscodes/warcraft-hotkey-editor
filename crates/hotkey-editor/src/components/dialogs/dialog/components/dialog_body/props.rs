use dioxus::prelude::*;

use crate::components::dialogs::dialog::DialogProps;

/// The body's only input: the content to scroll. Forwarded from the dialog.
#[derive(Props, Clone, PartialEq)]
pub struct DialogBodyProps {
    pub children: Element,
}

impl From<&DialogProps> for DialogBodyProps {
    fn from(props: &DialogProps) -> Self {
        let children = props.children.clone();
        Self { children }
    }
}
