mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::help_dialog::HelpDialog;
use dioxus::prelude::*;
use presentation::{BurgerOnboardingPresentation, use_burger_onboarding};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BurgerOnboarding() -> Element {
    let BurgerOnboardingPresentation {
        open,
        on_open_change,
    } = use_burger_onboarding();
    rsx! {
        div {
            class: CLASS,
            HelpDialog {
                open,
                on_open_change,
            }
        }
    }
}

assert_component!(BurgerOnboarding);
