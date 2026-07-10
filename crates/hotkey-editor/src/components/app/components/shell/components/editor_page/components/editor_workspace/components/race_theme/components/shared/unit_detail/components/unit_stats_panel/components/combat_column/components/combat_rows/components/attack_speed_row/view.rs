use warcraft_keybinds::AttackSpeed;

/// The published `View` contract mirroring [`AttackSpeedRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AttackSpeedRowView {
    pub value: AttackSpeed,
}

impl ddd::View for AttackSpeedRowView {}
