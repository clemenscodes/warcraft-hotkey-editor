#[derive(Clone, PartialEq)]
pub struct UnitDetailDialogPortraitSlotView {
    pub portrait_url: Option<String>,
}

impl ddd::Layered for UnitDetailDialogPortraitSlotView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitDetailDialogPortraitSlotView {}
