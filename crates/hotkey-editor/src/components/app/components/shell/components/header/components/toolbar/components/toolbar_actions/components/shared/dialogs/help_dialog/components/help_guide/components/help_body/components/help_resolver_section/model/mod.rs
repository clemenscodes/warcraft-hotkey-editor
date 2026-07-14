use super::view::HelpResolverSectionView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpGlossaryItem;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpResolverSectionModel {
    pub prose: &'static [&'static str],
    pub glossary: &'static [&'static [HelpGlossaryItem]],
}

impl From<&HelpResolverSectionView> for HelpResolverSectionModel {
    fn from(view: &HelpResolverSectionView) -> Self {
        let HelpResolverSectionView { prose, glossary } = view.clone();
        Self { prose, glossary }
    }
}

impl ddd::Model for HelpResolverSectionModel {
    type View = HelpResolverSectionView;
}
