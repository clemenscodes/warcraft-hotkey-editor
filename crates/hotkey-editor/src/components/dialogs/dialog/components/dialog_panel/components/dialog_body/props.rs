use dioxus::prelude::*;

use super::super::super::DialogPanelProps;

/// The body's only input: the content to scroll. Forwarded from the panel.
#[derive(Props, Clone, PartialEq)]
pub struct DialogBodyProps {
    pub children: Element,
}

impl From<&DialogPanelProps> for DialogBodyProps {
    fn from(props: &DialogPanelProps) -> Self {
        let children = props.children.clone();
        Self { children }
    }
}
