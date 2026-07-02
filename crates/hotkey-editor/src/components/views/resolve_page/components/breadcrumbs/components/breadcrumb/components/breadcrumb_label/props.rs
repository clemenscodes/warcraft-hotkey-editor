use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbLabelProps {
    #[props(into)]
    pub text: String,
}
