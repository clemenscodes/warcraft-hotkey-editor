use crate::services::customkeys::queries::unit_override_target_query::UnitOverrideTargetView;

#[derive(Clone, PartialEq)]
pub struct HotkeyOverrideSectionView {
    pub(crate) override_target: UnitOverrideTargetView,
}

impl ddd::Layered for HotkeyOverrideSectionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HotkeyOverrideSectionView {}
