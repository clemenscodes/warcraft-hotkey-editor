use super::components::info_actions::{InfoActions, InfoActionsProps};
use super::components::info_content::{InfoContent, InfoContentProps};
use super::props::InfoDialogConfig;
use crate::components::dialogs::dialog::DialogProps;
use dioxus::prelude::*;

impl From<&InfoDialogConfig> for DialogProps {
    fn from(props: &InfoDialogConfig) -> Self {
        let open = props.open;
        let title = props.title.to_owned();
        let content = InfoContentProps::from(props);
        let actions = InfoActionsProps::from(props);
        let children = rsx! {
            InfoContent { ..content }
            InfoActions { ..actions }
        };
        Self {
            open,
            title,
            children,
            footer: None,
            on_open_change: None,
        }
    }
}
