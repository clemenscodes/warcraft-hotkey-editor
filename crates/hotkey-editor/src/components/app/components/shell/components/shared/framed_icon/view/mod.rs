use super::icon_radius::IconRadius;

#[derive(Clone, PartialEq)]
pub struct FramedIconView {
    pub src: Option<String>,
    pub alt: String,
    pub radius: IconRadius,
    pub hover_glow: bool,
    pub placeholder: bool,
}

impl ddd::View for FramedIconView {}
