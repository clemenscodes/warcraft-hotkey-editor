use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

/// The footer's only input: the optional action bar content, forwarded from the
/// dialog. `None` when the dialog has no footer.
#[derive(Props, Clone, PartialEq)]
pub struct DialogFooterProps {
    pub footer: Option<Element>,
}

impl From<&DialogProps> for DialogFooterProps {
    fn from(props: &DialogProps) -> Self {
        let footer = props.footer.clone();
        Self { footer }
    }
}
