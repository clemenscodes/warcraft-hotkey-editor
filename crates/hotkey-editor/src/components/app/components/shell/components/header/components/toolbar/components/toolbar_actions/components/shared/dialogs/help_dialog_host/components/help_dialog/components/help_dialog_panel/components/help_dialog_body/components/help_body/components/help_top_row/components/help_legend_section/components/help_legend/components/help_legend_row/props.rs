use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpLegendEntry;
use dioxus::prelude::*;

/// One legend row's only input: the legend entry to render.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendRowProps {
    pub entry: HelpLegendEntry,
}
