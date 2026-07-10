use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog_host::components::help_dialog::data::HelpLegendEntry;
use dioxus::prelude::*;

/// The legend's input: the toolbar rows to render.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendProps {
    pub rows: &'static [HelpLegendEntry],
}
