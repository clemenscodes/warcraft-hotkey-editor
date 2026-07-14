use super::view::PanelCardView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

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
