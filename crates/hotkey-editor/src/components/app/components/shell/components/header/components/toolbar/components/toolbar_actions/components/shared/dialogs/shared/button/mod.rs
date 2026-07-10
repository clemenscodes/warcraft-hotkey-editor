pub mod components;
mod props;
mod view;

pub use view::ButtonView;
mod state;

use components::primary_button::PrimaryButton;
use components::secondary_button::SecondaryButton;
use dioxus::prelude::*;
use props::ButtonProps;
pub use state::ButtonVariant;
use tw_macro::assert_component;

/// A WC3 action button in primary or secondary weight. A pure dispatcher: from the
/// variant it renders `PrimaryButton` xor `SecondaryButton`. Each look owns its own
/// classed `button` root and forwards one click; callers pick the variant and pass
/// the label text.
#[component]
pub fn Button(props: ButtonProps) -> Element {
    match props.variant {
        ButtonVariant::Primary => {
            let onclick = props.onclick;
            let label = props.label.clone();
            rsx! {
                PrimaryButton { onclick, label }
            }
        }
        ButtonVariant::Secondary => {
            let onclick = props.onclick;
            let label = props.label.clone();
            rsx! {
                SecondaryButton { onclick, label }
            }
        }
    }
}

assert_component!(Button);
