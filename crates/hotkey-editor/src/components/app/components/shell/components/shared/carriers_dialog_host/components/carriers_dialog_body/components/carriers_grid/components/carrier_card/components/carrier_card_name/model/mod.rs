use super::view::CarrierCardNameView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardNameModel {
    #[props(into)]
    pub text: String,
}

impl From<&CarrierCardNameView> for CarrierCardNameModel {
    fn from(view: &CarrierCardNameView) -> Self {
        let CarrierCardNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Model for CarrierCardNameModel {
    type View = CarrierCardNameView;
}
