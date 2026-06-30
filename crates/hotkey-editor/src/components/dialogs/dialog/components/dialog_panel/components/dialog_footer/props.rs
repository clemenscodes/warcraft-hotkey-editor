use dioxus::prelude::*;

use super::super::super::DialogPanelProps;

/// The footer's only input: the optional action bar content, forwarded from the
/// panel. `None` when the dialog has no footer.
#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterProps {
    pub footer: Option<Element>,
}

impl From<&DialogPanelProps> for DialogFooterProps {
    fn from(props: &DialogPanelProps) -> Self {
        let footer = props.footer.clone();
        Self { footer }
    }
}
