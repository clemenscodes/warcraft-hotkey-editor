use super::view::ResolveSectionTabCountView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResolveSectionTabCountModel {
    pub count: usize,
}

impl From<&ResolveSectionTabCountView> for ResolveSectionTabCountModel {
    fn from(view: &ResolveSectionTabCountView) -> Self {
        let ResolveSectionTabCountView { count } = view.clone();
        Self { count }
    }
}

impl ddd::Model for ResolveSectionTabCountModel {
    type View = ResolveSectionTabCountView;
}
