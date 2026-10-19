use super::view::PrimaryAttributeRowView;
use dioxus::prelude::*;
use warcraft_api::{AttributeStatistic, Gain};

#[derive(Props, Clone, PartialEq)]
pub struct PrimaryAttributeRowModel {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&PrimaryAttributeRowView> for PrimaryAttributeRowModel {
    fn from(view: &PrimaryAttributeRowView) -> Self {
        let PrimaryAttributeRowView {
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

impl ddd::Layered for PrimaryAttributeRowModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for PrimaryAttributeRowModel {
    type View = PrimaryAttributeRowView;
}
