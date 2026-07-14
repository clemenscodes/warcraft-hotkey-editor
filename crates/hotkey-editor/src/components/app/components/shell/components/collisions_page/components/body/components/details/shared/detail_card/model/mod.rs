use super::view::DetailCardView;
use browser_kit::frame::Render;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DetailCardModel<Body: Render<Output = Element>> {
    pub body: Body,
}

impl<Body: Render<Output = Element>> From<&DetailCardView<Body>> for DetailCardModel<Body> {
    fn from(view: &DetailCardView<Body>) -> Self {
        let DetailCardView { body } = view.clone();
        Self { body }
    }
}

impl<Body: Render<Output = Element>> ddd::Model for DetailCardModel<Body> {
    type View = DetailCardView<Body>;
}
