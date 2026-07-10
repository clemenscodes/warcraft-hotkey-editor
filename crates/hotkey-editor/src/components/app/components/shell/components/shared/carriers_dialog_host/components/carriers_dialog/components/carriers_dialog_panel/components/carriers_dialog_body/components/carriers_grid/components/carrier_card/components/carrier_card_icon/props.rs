use super::view::CarrierCardIconView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardIconProps {
    pub src: Option<String>,
    #[props(into)]
    pub alt: String,
}

impl From<&CarrierCardIconView> for CarrierCardIconProps {
    fn from(view: &CarrierCardIconView) -> Self {
        let CarrierCardIconView { src, alt } = view.clone();
        Self { src, alt }
    }
}

impl ddd::Props for CarrierCardIconProps {
    type View = CarrierCardIconView;
}
