use dioxus::prelude::*;

/// The alt-state block wraps a header and any description lines passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateContainerProps {
    pub children: Element,
}
