use super::view::HelpResolverSectionView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::components::help_guide::data::HelpGlossaryItem;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HelpResolverSectionModel {
    pub glossary: &'static [&'static [HelpGlossaryItem]],
}

impl From<&HelpResolverSectionView> for HelpResolverSectionModel {
    fn from(view: &HelpResolverSectionView) -> Self {
        let HelpResolverSectionView { glossary } = view.clone();
        Self { glossary }
    }
}

impl ddd::Layered for HelpResolverSectionModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for HelpResolverSectionModel {
    type View = HelpResolverSectionView;
}
