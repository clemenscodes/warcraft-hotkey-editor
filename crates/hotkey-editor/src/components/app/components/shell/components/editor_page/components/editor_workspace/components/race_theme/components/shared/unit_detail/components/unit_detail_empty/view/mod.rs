#[derive(Clone, PartialEq)]
pub struct UnitDetailEmptyView {
    pub message: String,
}

impl ddd::Layered for UnitDetailEmptyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitDetailEmptyView {}
