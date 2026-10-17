#[derive(Clone, PartialEq)]
pub struct UnitDetailDialogIntroView {
    pub portrait_url: Option<String>,
    pub description_text: String,
}

impl ddd::Layered for UnitDetailDialogIntroView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for UnitDetailDialogIntroView {}
