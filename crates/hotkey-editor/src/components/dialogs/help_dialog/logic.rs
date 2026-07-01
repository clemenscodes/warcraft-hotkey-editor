use super::components::help_body::{HelpBody, HelpBodyProps};
use super::components::help_dismiss::{HelpDismiss, HelpDismissProps};
use super::data::HELP_CONTENT;
use super::props::HelpDialogProps;
use crate::components::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&HelpDialogProps> for DialogProps {
    fn from(props: &HelpDialogProps) -> Self {
        let open = props.help_open;
        let title = String::from("How to use this editor");
        let dismiss = HelpDismissProps::from(props);
        let body = HelpBodyProps {
            content: HELP_CONTENT,
        };
        let footer = Some(rsx! {
            HelpDismiss { ..dismiss }
        });
        let children = rsx! {
            HelpBody { ..body }
        };
        Self {
            open,
            title,
            children,
            footer,
            on_open_change: None,
        }
    }
}
