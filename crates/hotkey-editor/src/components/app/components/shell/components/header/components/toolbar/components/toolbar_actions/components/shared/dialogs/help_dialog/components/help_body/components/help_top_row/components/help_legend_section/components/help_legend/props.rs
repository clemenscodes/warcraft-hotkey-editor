use dioxus::prelude::*;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_body::components::help_top_row::components::help_legend_section::HelpLegendSectionProps;
use super::components::help_legend_row::HelpLegendRowProps;

/// The legend's input: the toolbar rows to render.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendProps {
    pub rows: &'static [HelpLegendRowProps],
}

impl From<&HelpLegendSectionProps> for HelpLegendProps {
    fn from(props: &HelpLegendSectionProps) -> Self {
        Self { rows: props.rows }
    }
}
