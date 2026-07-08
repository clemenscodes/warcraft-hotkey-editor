use super::components::info_actions::InfoActionsProps;
use super::components::info_content::InfoContentProps;
use dioxus::prelude::*;

/// The info dialog's scroll region input: the centered instruction content and
/// the trailing action row.
#[derive(Props, Clone, PartialEq)]
pub struct InfoDialogBodyProps {
    pub content: InfoContentProps,
    pub actions: InfoActionsProps,
}

impl From<&InfoDialogBodyProps> for InfoContentProps {
    fn from(props: &InfoDialogBodyProps) -> Self {
        props.content.clone()
    }
}

impl From<&InfoDialogBodyProps> for InfoActionsProps {
    fn from(props: &InfoDialogBodyProps) -> Self {
        props.actions.clone()
    }
}
