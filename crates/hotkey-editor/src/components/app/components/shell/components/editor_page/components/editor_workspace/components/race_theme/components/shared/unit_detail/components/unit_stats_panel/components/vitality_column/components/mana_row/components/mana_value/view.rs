use warcraft_keybinds::Mana;

/// The published `View` contract mirroring [`ManaValueProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ManaValueView {
    pub value: Mana,
}

impl ddd::View for ManaValueView {}
