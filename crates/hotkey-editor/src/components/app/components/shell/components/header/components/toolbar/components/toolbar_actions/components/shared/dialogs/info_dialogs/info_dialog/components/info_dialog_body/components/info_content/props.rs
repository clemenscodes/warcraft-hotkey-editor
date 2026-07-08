use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::InfoDialogConfig;
use dioxus::prelude::*;

/// The instruction block's content: the intro line and the optional warning. The
/// filename chip is fixed, so it is not carried here.
#[derive(Props, Clone, PartialEq)]
pub struct InfoContentProps {
    pub intro: &'static str,
    pub warning: Option<&'static str>,
}

impl From<&InfoDialogConfig> for InfoContentProps {
    fn from(props: &InfoDialogConfig) -> Self {
        let intro = props.intro;
        let warning = props.warning;
        Self { intro, warning }
    }
}
