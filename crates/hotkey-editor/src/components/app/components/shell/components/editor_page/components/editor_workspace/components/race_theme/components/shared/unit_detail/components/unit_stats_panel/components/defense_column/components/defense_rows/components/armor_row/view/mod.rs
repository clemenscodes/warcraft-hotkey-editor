use warcraft_keybinds::Armor;

/// The published `View` contract mirroring [`ArmorRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ArmorRowView {
    pub value: Armor,
}

impl ddd::View for ArmorRowView {}
