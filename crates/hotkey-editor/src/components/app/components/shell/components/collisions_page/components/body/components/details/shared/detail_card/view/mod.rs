use browser_kit::frame::Render;
use dioxus::prelude::*;

/// The published `View` contract for the shared collision-detail card: the caller's body
/// region, placed inside the shared bordered detail surface. Generic over the `Body` region
/// (a `Render`), so a detail's filled/empty content supplies itself while sharing the one
/// surface.
#[derive(Clone, PartialEq)]
pub struct DetailCardView<Body: Render<Output = Element>> {
    pub body: Body,
}

impl<Body: Render<Output = Element>> ddd::View for DetailCardView<Body> {}
