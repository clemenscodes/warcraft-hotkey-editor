use super::view::CarrierCardNameView;
use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CarrierCardNameProps {
    #[props(into)]
    pub text: String,
}

impl From<&CarrierCardNameView> for CarrierCardNameProps {
    fn from(view: &CarrierCardNameView) -> Self {
        let CarrierCardNameView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for CarrierCardNameProps {
    type View = CarrierCardNameView;
}
