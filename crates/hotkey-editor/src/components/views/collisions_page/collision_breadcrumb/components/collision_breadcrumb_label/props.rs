use dioxus::prelude::*;
#[derive(Props, Clone, PartialEq)]
pub struct CollisionBreadcrumbLabelProps {
    #[props(into)]
    pub text: String,
}
