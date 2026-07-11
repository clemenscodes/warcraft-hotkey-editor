use super::view::StatIconImgView;
use dioxus::prelude::*;

/// A stat column's icon image: the asset source and its alt text.
#[derive(Props, Clone, PartialEq)]
pub struct StatIconImgModel {
    pub src: Asset,
    #[props(into)]
    pub alt: String,
}

impl From<&StatIconImgView> for StatIconImgModel {
    fn from(view: &StatIconImgView) -> Self {
        let StatIconImgView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for StatIconImgModel {
    type View = StatIconImgView;
}
