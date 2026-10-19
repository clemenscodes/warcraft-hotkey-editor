#[derive(Clone, PartialEq)]
pub struct HelpSectionTitleView {
    pub title: String,
}

impl ddd::Layered for HelpSectionTitleView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HelpSectionTitleView {}
