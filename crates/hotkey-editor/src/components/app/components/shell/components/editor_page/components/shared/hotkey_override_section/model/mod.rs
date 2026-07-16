use super::view::HotkeyOverrideSectionView;
use crate::services::customkeys::queries::unit_override_target_query::UnitOverrideTargetView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyOverrideSectionModel {
    pub(crate) override_target: UnitOverrideTargetView,
}

impl From<&HotkeyOverrideSectionView> for HotkeyOverrideSectionModel {
    fn from(view: &HotkeyOverrideSectionView) -> Self {
        let HotkeyOverrideSectionView { override_target } = view.clone();
        Self { override_target }
    }
}

impl ddd::Model for HotkeyOverrideSectionModel {
    type View = HotkeyOverrideSectionView;
}
