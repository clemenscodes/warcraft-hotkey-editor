use super::components::help_body::HelpBodyProps;
use super::components::help_dismiss::HelpDismissProps;
use dioxus::prelude::*;

/// The help dialog's scroll region input: the guide body content and the
/// dismiss button beneath it.
#[derive(Props, Clone, PartialEq)]
pub struct HelpDialogBodyProps {
    pub body: HelpBodyProps,
    pub dismiss: HelpDismissProps,
}

impl From<&HelpDialogBodyProps> for HelpBodyProps {
    fn from(props: &HelpDialogBodyProps) -> Self {
        props.body.clone()
    }
}

impl From<&HelpDialogBodyProps> for HelpDismissProps {
    fn from(props: &HelpDialogBodyProps) -> Self {
        props.dismiss.clone()
    }
}
