#[derive(Clone, PartialEq)]
pub struct PagerSpacerView {
    pub height_px: i32,
}

impl ddd::Layered for PagerSpacerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PagerSpacerView {}
