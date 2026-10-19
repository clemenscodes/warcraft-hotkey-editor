#[derive(Clone, PartialEq)]
pub struct FightIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::Layered for FightIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for FightIconView {}
