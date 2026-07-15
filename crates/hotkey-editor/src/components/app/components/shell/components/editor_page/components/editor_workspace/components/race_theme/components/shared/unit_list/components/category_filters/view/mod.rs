use warcraft_api::UnitKind;

#[derive(Clone, PartialEq)]
pub struct CategoryFiltersView {
    pub tabs: Vec<UnitKind>,
}

impl ddd::View for CategoryFiltersView {}
