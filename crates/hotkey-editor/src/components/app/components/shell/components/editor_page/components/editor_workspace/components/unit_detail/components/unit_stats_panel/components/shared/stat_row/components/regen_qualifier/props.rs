use dioxus::prelude::*;

/// The regen qualifier ("at night" / "on blight") shown before the regen gain.
#[derive(Props, Clone, PartialEq)]
pub struct RegenQualifierProps {
    #[props(default)]
    pub text: Option<&'static str>,
}
