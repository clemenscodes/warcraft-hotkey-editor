use super::icon_radius::IconRadius;
use super::view::FramedIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FramedIconModel {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
    pub radius: IconRadius,
    pub hover_glow: bool,
    pub placeholder: bool,
}

impl From<&FramedIconView> for FramedIconModel {
    fn from(view: &FramedIconView) -> Self {
        let FramedIconView {
            src,
            alt,
            radius,
            hover_glow,
            placeholder,
        } = view.clone();
        Self {
            src,
            alt,
            radius,
            hover_glow,
            placeholder,
        }
    }
}

impl ddd::Model for FramedIconModel {
    type View = FramedIconView;
}
