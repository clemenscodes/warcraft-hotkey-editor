use super::state::AbilityPair;
use super::view::ConflictPairRowView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ConflictPairRowModel {
    pub pair: Option<AbilityPair>,
}

impl From<&ConflictPairRowView> for ConflictPairRowModel {
    fn from(view: &ConflictPairRowView) -> Self {
        let ConflictPairRowView { pair } = view.clone();
        Self { pair }
    }
}

impl ddd::Model for ConflictPairRowModel {
    type View = ConflictPairRowView;
}
