use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct ResolveBreadcrumbLabelProps {
    #[props(into)]
    pub text: String,
}
