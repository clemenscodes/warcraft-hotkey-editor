use super::view::{SegmentChoice, SegmentedControlView};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SegmentedControlModel {
    pub options: Vec<SegmentChoice>,
}

impl From<&SegmentedControlView> for SegmentedControlModel {
    fn from(view: &SegmentedControlView) -> Self {
        let SegmentedControlView { options } = view.clone();
        Self { options }
    }
}

impl ddd::Model for SegmentedControlModel {
    type View = SegmentedControlView;
}
