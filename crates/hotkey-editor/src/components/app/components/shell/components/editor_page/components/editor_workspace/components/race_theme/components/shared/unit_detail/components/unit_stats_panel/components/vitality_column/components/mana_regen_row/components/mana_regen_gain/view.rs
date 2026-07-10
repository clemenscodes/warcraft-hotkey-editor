use warcraft_keybinds::ManaRegen;

/// The published `View` contract mirroring [`ManaRegenGainProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ManaRegenGainView {
    pub value: ManaRegen,
}

impl ddd::View for ManaRegenGainView {}
