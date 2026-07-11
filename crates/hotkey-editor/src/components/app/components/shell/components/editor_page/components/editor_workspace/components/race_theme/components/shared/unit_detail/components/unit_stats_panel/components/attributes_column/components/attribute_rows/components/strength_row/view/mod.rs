use warcraft_keybinds::AttributeStatistic;

/// The published `View` contract mirroring [`StrengthRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct StrengthRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::View for StrengthRowView {}
