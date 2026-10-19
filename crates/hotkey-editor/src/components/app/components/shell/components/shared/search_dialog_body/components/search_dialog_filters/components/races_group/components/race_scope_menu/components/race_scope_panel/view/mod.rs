#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct RaceScopePanelView {
    pub leaving: bool,
}

impl ddd::Layered for RaceScopePanelView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RaceScopePanelView {}
