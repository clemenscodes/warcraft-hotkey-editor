pub mod components;
mod model;
mod view;

pub use view::ButtonView;
mod state;

use components::primary_button::PrimaryButton;
use components::secondary_button::SecondaryButton;
use dioxus::prelude::*;
use model::ButtonModel;
pub use state::ButtonVariant;
use tw_macro::assert_component;

#[component]
pub fn Button(props: ButtonModel) -> Element {
    match props.variant {
        ButtonVariant::Primary => {
            let onclick = props.onclick;
            let label = props.label.clone();
            rsx! {
                PrimaryButton {
                    onclick,
                    label,
                }
            }
        }
        ButtonVariant::Secondary => {
            let onclick = props.onclick;
            let label = props.label.clone();
            rsx! {
                SecondaryButton {
                    onclick,
                    label,
                }
            }
        }
    }
}

assert_component!(Button);
