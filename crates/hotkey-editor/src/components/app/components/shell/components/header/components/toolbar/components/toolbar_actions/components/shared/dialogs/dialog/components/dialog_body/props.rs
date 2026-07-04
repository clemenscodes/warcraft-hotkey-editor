use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

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
