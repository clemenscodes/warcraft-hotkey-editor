use dioxus::prelude::*;

/// The label column wraps the (optional) alt-state label passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateHeaderTextProps {
    pub children: Element,
}
