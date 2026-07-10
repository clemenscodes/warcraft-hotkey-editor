use dioxus::prelude::*;

/// One legend row's inputs: the toolbar glyph, the button's name, and its
/// one-line description.
#[derive(Props, Clone, PartialEq)]
pub struct HelpLegendRowProps {
    pub icon: &'static str,
    pub label: &'static str,
    pub description: &'static str,
}
