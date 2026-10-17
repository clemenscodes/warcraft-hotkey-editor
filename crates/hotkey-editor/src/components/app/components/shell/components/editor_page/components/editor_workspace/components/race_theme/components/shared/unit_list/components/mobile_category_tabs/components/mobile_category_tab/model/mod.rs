use super::view::MobileCategoryTabView;
use dioxus::prelude::*;
use warcraft_api::UnitKind;

#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabModel {
    pub kind: UnitKind,
}

impl From<&MobileCategoryTabView> for MobileCategoryTabModel {
    fn from(view: &MobileCategoryTabView) -> Self {
        let MobileCategoryTabView { kind } = view.clone();
        Self { kind }
    }
}

impl ddd::Layered for MobileCategoryTabModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for MobileCategoryTabModel {
    type View = MobileCategoryTabView;
}
