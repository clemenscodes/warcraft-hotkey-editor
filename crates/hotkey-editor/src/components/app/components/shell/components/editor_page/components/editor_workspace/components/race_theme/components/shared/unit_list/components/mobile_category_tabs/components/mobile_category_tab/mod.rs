pub mod components;
mod hooks;
mod logic;
mod props;

use components::active_mobile_category_tab::{
    ActiveMobileCategoryTab, ActiveMobileCategoryTabProps,
};
use components::idle_mobile_category_tab::{IdleMobileCategoryTab, IdleMobileCategoryTabProps};
use dioxus::prelude::*;
use hooks::{MobileCategoryTabView, use_mobile_category_tab};
pub use props::MobileCategoryTabProps;
use tw_macro::assert_component;

/// A single category tab in the mobile unit picker. A pure dispatcher: from whether its
/// kind is the active category (read from editor context) it renders
/// `ActiveMobileCategoryTab` xor `IdleMobileCategoryTab`. Each owns its `<button>` and
/// its own look — the active one wears the race accent read from the theme's
/// `--race-accent`; there is no `data-active`, the look follows the component.
#[component]
pub fn MobileCategoryTab(props: MobileCategoryTabProps) -> Element {
    let MobileCategoryTabView { is_active, model } = use_mobile_category_tab(props.kind);
    match is_active {
        true => {
            let tab = ActiveMobileCategoryTabProps::from(&model);
            rsx! {
                ActiveMobileCategoryTab { ..tab }
            }
        }
        false => {
            let tab = IdleMobileCategoryTabProps::from(&model);
            rsx! {
                IdleMobileCategoryTab { ..tab }
            }
        }
    }
}

assert_component!(MobileCategoryTab);
