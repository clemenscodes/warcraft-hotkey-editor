use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpGlossaryItem;

#[derive(Clone, PartialEq)]
pub struct HelpResolverSectionView {
    pub prose: &'static [&'static str],
    pub glossary: &'static [&'static [HelpGlossaryItem]],
}

impl ddd::View for HelpResolverSectionView {}
