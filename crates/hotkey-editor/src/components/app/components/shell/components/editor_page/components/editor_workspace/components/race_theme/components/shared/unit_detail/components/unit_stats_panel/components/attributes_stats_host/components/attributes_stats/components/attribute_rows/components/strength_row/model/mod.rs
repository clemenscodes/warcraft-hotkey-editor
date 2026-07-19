use super::view::StrengthRowView;
use dioxus::prelude::*;
use warcraft_api::AttributeStatistic;

#[derive(Props, Clone, PartialEq)]
pub struct StrengthRowModel {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl From<&StrengthRowView> for StrengthRowModel {
    fn from(view: &StrengthRowView) -> Self {
        let StrengthRowView {
            statistic,
            is_primary,
        } = view.clone();
        Self {
            statistic,
            is_primary,
        }
    }
}

impl ddd::Model for StrengthRowModel {
    type View = StrengthRowView;
}
