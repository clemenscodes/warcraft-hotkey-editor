#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ActiveAccentView {
    pub hidden: bool,
}

impl ddd::Layered for ActiveAccentView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveAccentView {}
