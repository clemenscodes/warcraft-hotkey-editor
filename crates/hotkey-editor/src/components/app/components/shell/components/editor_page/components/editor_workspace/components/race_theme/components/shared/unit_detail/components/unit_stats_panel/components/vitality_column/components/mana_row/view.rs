use warcraft_keybinds::Mana;

/// The published `View` contract mirroring [`ManaRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ManaRowView {
    pub value: Mana,
}

impl ddd::View for ManaRowView {}
