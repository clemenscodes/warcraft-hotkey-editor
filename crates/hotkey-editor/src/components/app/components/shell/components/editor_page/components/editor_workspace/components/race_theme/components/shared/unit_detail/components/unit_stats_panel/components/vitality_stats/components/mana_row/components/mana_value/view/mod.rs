use warcraft_api::Mana;

/// The published `View` contract mirroring [`ManaValueModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ManaValueView {
    pub value: Mana,
}

impl ddd::View for ManaValueView {}
