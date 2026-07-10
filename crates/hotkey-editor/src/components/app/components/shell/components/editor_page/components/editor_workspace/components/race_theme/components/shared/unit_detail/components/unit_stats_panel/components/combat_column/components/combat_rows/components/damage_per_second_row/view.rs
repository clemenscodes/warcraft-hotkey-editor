use warcraft_keybinds::DamagePerSecond;

/// The published `View` contract mirroring [`DamagePerSecondRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DamagePerSecondRowView {
    pub value: Option<DamagePerSecond>,
}

impl ddd::View for DamagePerSecondRowView {}
