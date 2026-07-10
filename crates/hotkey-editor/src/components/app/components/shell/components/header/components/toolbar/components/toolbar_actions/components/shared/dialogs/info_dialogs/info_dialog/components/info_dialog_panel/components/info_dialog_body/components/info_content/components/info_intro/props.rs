use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::info_dialog::components::info_dialog_panel::components::info_dialog_body::components::info_content::InfoContentProps;
use dioxus::prelude::*;

/// The intro line's copy, taken from the content block's props.
#[derive(Props, Clone, PartialEq)]
pub struct InfoIntroProps {
    pub intro: &'static str,
}

impl From<&InfoContentProps> for InfoIntroProps {
    fn from(props: &InfoContentProps) -> Self {
        let intro = props.intro;
        Self { intro }
    }
}
