pub mod components;
mod logic;
mod props;

use components::primary_button::{PrimaryButton, PrimaryButtonProps};
use components::secondary_button::{SecondaryButton, SecondaryButtonProps};
use dioxus::prelude::*;
pub use props::{ButtonProps, ButtonVariant};
use tw_macro::assert_component;

/// A WC3 action button in primary or secondary weight. A pure dispatcher: from the
/// variant it renders `PrimaryButton` xor `SecondaryButton`. Each look owns its own
/// classed `button` root and forwards one click; callers pick the variant and pass
/// the label text.
#[component]
pub fn Button(props: ButtonProps) -> Element {
    match props.variant {
        ButtonVariant::Primary => {
            let button = PrimaryButtonProps::from(&props);
            rsx! {
                PrimaryButton { ..button }
            }
        }
        ButtonVariant::Secondary => {
            let button = SecondaryButtonProps::from(&props);
            rsx! {
                SecondaryButton { ..button }
            }
        }
    }
}

assert_component!(Button);
