use dioxus::prelude::*;

/// The rows column beside a stat column's icon.
#[derive(Props, Clone, PartialEq)]
pub struct StatRowsProps {
    pub children: Element,
}
