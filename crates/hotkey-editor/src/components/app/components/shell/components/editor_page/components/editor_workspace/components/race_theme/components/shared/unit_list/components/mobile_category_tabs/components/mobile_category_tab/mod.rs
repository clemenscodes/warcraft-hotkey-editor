pub mod components;
mod hooks;
mod logic;
mod props;
mod view;

pub use view::MobileCategoryTabView;

use components::active_mobile_category_tab::ActiveMobileCategoryTab;
use components::idle_mobile_category_tab::IdleMobileCategoryTab;
use dioxus::prelude::*;
use hooks::{MobileCategoryTabDispatch, use_mobile_category_tab};
use props::MobileCategoryTabProps;
use tw_macro::assert_component;

/// A single category tab in the mobile unit picker. A pure dispatcher: from whether its
/// kind is the active category (read from editor context) it renders
/// `ActiveMobileCategoryTab` xor `IdleMobileCategoryTab`. Each owns its `<button>` and
/// its own look — the active one wears the race accent read from the theme's
/// `--race-color`; there is no `data-active`, the look follows the component.
#[component]
pub fn MobileCategoryTab(props: MobileCategoryTabProps) -> Element {
    let MobileCategoryTabDispatch { is_active, model } = use_mobile_category_tab(props.kind);
    let label = model.label();
    let onclick = model.onclick();
    match is_active {
        true => rsx! {
            ActiveMobileCategoryTab { label, onclick }
        },
        false => rsx! {
            IdleMobileCategoryTab { label, onclick }
        },
    }
}

assert_component!(MobileCategoryTab);
