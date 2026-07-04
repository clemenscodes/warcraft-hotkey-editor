use dioxus::prelude::*;

/// The current hero level shown in the centre of the trigger.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelTriggerNumberProps {
    #[props(into)]
    pub number: String,
}
