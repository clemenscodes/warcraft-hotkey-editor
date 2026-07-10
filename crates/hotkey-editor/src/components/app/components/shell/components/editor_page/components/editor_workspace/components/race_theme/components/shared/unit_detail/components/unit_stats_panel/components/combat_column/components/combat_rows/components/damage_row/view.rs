use warcraft_keybinds::DamageRange;

/// The published `View` contract mirroring [`DamageRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DamageRowView {
    pub value: DamageRange,
}

impl ddd::View for DamageRowView {}
