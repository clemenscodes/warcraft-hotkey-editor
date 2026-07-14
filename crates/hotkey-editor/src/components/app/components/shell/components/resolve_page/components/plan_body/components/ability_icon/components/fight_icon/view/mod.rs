#[derive(Clone, PartialEq)]
pub struct FightIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for FightIconView {}
