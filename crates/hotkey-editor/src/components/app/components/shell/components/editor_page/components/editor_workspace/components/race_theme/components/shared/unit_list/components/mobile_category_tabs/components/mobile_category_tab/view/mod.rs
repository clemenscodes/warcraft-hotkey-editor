use warcraft_api::UnitKind;

#[derive(Clone, PartialEq)]
pub struct MobileCategoryTabView {
    pub kind: UnitKind,
}

impl ddd::View for MobileCategoryTabView {}
