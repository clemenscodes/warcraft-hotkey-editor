use dioxus::prelude::*;

/// The picker body wraps the explainer and grid anchor passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerBodyProps {
    pub children: Element,
}
