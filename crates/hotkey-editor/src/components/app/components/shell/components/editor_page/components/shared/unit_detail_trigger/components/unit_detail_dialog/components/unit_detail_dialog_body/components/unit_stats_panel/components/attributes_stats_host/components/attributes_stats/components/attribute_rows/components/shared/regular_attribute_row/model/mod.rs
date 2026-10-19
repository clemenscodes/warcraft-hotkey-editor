use super::view::RegularAttributeRowView;
use dioxus::prelude::*;
use warcraft_api::{AttributeStatistic, Gain};

#[derive(Props, Clone, PartialEq)]
pub struct RegularAttributeRowModel {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&RegularAttributeRowView> for RegularAttributeRowModel {
    fn from(view: &RegularAttributeRowView) -> Self {
        let RegularAttributeRowView {
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

impl ddd::Layered for RegularAttributeRowModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for RegularAttributeRowModel {
    type View = RegularAttributeRowView;
}
