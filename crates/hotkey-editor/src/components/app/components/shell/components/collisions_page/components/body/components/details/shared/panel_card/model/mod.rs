use super::view::PanelCardView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The shared conflict-panel card's own props: the caller-supplied body region placed inside
/// the shared tinted, bordered conflict surface. Generic over the body `Body` region — the
/// surface is the card's own chrome, not a field.
#[derive(Props, Clone, PartialEq)]
pub struct PanelCardModel<Body: Render<Output = Element>> {
    pub body: Body,
}

impl<Body: Render<Output = Element>> From<&PanelCardView<Body>> for PanelCardModel<Body> {
    fn from(view: &PanelCardView<Body>) -> Self {
        let PanelCardView { body } = view.clone();
        Self { body }
    }
}

impl<Body: Render<Output = Element>> ddd::Model for PanelCardModel<Body> {
    type View = PanelCardView<Body>;
}
