use warcraft_api::Evasion;

/// The published `View` contract mirroring [`EvasionRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct EvasionRowView {
    pub value: Evasion,
}

impl ddd::View for EvasionRowView {}
