use super::view::CarrierCardIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardIconModel {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&CarrierCardIconView> for CarrierCardIconModel {
    fn from(view: &CarrierCardIconView) -> Self {
        let CarrierCardIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Model for CarrierCardIconModel {
    type View = CarrierCardIconView;
}
