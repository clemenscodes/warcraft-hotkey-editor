use dioxus::prelude::*;

/// The regen qualifier ("at night" / "on blight") shown before the regen gain.
#[derive(Props, Clone, PartialEq)]
pub struct RegenQualifierProps {
    #[props(into)]
    pub text: String,
}
