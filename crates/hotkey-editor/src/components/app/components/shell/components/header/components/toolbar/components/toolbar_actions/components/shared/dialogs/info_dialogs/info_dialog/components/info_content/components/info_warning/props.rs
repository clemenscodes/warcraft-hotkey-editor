use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::components::info_content::InfoContentProps;
use dioxus::prelude::*;

/// The warning callout's copy, taken from the content block's props. `None` for
/// dialogs without a warning, where nothing renders.
#[derive(Props, Clone, PartialEq)]
pub struct InfoWarningProps {
    pub warning: Option<&'static str>,
}

impl From<&InfoContentProps> for InfoWarningProps {
    fn from(props: &InfoContentProps) -> Self {
        let warning = props.warning;
        Self { warning }
    }
}
