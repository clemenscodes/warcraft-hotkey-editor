use super::data::LABEL;
use super::model::BurgerResolveItemModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::shared::icons::ICON_RESOLVE;
use crate::services::customkeys::context::use_custom_keys_service;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use dioxus::prelude::*;

pub struct BurgerResolveItemPresentation {
    pub(super) icon: &'static str,
    pub(super) label: String,
    pub(super) state: BurgerItemState,
    pub(super) disabled: bool,
    pub(super) role: Option<&'static str>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_burger_resolve_item(
    props: &BurgerResolveItemModel,
) -> BurgerResolveItemPresentation {
    let navigation = use_view_navigation();
    let custom_keys_service = use_custom_keys_service();
    let keys = custom_keys_service.keys();
    let has_file_memo = use_memo(move || keys.read().is_some());
    let has_file = has_file_memo();
    let disabled = !has_file;
    let on_close = props.on_close;
    let onclick = EventHandler::new(move |event: MouseEvent| {
        navigation.apply(AppView::Resolve);
        on_close.call(event);
    });
    BurgerResolveItemPresentation {
        icon: ICON_RESOLVE,
        label: String::from(LABEL),
        state: BurgerItemState::Idle,
        disabled,
        role: Some("menuitem"),
        onclick,
    }
}

impl ddd::Presentation for BurgerResolveItemPresentation {
    type Model = BurgerResolveItemModel;
}
