use warcraft_api::UnitKind;

#[derive(Clone, PartialEq)]
pub struct CategoryScrollView {
    pub sections: Vec<UnitKind>,
}

impl ddd::View for CategoryScrollView {}
