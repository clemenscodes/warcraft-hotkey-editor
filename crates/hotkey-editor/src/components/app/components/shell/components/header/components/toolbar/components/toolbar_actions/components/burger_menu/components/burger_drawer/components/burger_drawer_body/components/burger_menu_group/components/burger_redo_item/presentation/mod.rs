use super::data::LABEL;
use super::model::BurgerRedoItemModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::shared::icons::ICON_REDO;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

pub struct BurgerRedoItemPresentation {
    pub(super) icon: &'static str,
    pub(super) label: String,
    pub(super) state: BurgerItemState,
    pub(super) disabled: bool,
    pub(super) role: Option<&'static str>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_burger_redo_item(props: &BurgerRedoItemModel) -> BurgerRedoItemPresentation {
    let history = use_undo_history();
    let can_redo = history.can_redo();
    let disabled = !can_redo;
    let on_close = props.on_close;
    let onclick = EventHandler::new(move |event: MouseEvent| {
        history.redo();
        on_close.call(event);
    });
    BurgerRedoItemPresentation {
        icon: ICON_REDO,
        label: String::from(LABEL),
        state: BurgerItemState::Idle,
        disabled,
        role: Some("menuitem"),
        onclick,
    }
}

impl ddd::Presentation for BurgerRedoItemPresentation {
    type Model = BurgerRedoItemModel;
}
