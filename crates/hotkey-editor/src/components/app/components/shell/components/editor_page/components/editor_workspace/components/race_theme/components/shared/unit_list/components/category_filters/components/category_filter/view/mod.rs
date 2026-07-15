use warcraft_api::UnitKind;

#[derive(Clone, PartialEq)]
pub struct CategoryFilterView {
    pub kind: UnitKind,
}

impl ddd::View for CategoryFilterView {}
