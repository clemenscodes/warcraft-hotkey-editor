use dioxus::prelude::*;
/// A single "Column N" / "Row N" coordinate label on an island card.
#[derive(Props, Clone, PartialEq)]
pub struct IslandCoordProps {
    #[props(into)]
    pub text: String,
}
