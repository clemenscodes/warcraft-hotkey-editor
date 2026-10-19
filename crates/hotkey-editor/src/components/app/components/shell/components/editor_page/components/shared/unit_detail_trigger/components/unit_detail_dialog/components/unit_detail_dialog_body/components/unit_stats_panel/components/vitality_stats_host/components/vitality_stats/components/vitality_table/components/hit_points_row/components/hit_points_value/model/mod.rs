use super::view::HitPointsValueView;
use dioxus::prelude::*;
use warcraft_api::HitPoints;

#[derive(Props, Clone, PartialEq)]
pub struct HitPointsValueModel {
    pub value: HitPoints,
}

impl From<&HitPointsValueView> for HitPointsValueModel {
    fn from(view: &HitPointsValueView) -> Self {
        let HitPointsValueView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Layered for HitPointsValueModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for HitPointsValueModel {
    type View = HitPointsValueView;
}
