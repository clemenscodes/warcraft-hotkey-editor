use dioxus::prelude::*;
use crate::components::dialogs::help_dialog::components::help_body::components::help_top_row::HelpTopRowProps;
use crate::components::dialogs::help_dialog::components::help_body::components::help_top_row::components::help_legend_section::components::help_legend::components::help_legend_row::HelpLegendRowProps;

/// The legend column's input: the toolbar rows to lay out.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendSectionProps {
    pub rows: &'static [HelpLegendRowProps],
}

impl From<&HelpTopRowProps> for HelpLegendSectionProps {
    fn from(props: &HelpTopRowProps) -> Self {
        Self {
            rows: props.content.legend,
        }
    }
}
