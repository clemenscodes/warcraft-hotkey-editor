use super::view::PrimaryStrengthRowView;
use dioxus::prelude::*;
use warcraft_api::{AttributeStatistic, Gain};

#[derive(Props, Clone, PartialEq)]
pub struct PrimaryStrengthRowModel {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&PrimaryStrengthRowView> for PrimaryStrengthRowModel {
    fn from(view: &PrimaryStrengthRowView) -> Self {
        let PrimaryStrengthRowView {
            statistic,
            growth,
            label,
        } = view.clone();
        Self {
            statistic,
            growth,
            label,
        }
    }
}

impl ddd::Model for PrimaryStrengthRowModel {
    type View = PrimaryStrengthRowView;
}
