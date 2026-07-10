use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpLegendEntry;
use dioxus::prelude::*;

/// The legend column's input: the toolbar rows to lay out.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendSectionProps {
    pub rows: &'static [HelpLegendEntry],
}
