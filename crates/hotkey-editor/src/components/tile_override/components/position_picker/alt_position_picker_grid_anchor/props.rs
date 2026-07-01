use dioxus::prelude::*;

/// The grid anchor wraps the embedded command grid editor passed as children.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerGridAnchorProps {
    pub children: Element,
}
