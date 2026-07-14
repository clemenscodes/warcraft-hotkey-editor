use super::data::LABEL;
use super::model::BurgerUndoItemModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::components::burger_drawer::components::burger_drawer_body::components::shared::burger_menu_item::BurgerItemState;
use crate::components::app::components::shell::components::shared::icons::ICON_UNDO;
use crate::services::undo::context::use_undo_history;
use dioxus::prelude::*;

pub struct BurgerUndoItemPresentation {
    pub(super) icon: &'static str,
    pub(super) label: String,
    pub(super) state: BurgerItemState,
    pub(super) disabled: bool,
    pub(super) role: Option<&'static str>,
    pub(super) onclick: EventHandler<MouseEvent>,
}

pub(super) fn use_burger_undo_item(props: &BurgerUndoItemModel) -> BurgerUndoItemPresentation {
    let history = use_undo_history();
    let can_undo = history.can_undo();
    let disabled = !can_undo;
    let on_close = props.on_close;
    let onclick = EventHandler::new(move |event: MouseEvent| {
        history.undo();
        on_close.call(event);
    });
    BurgerUndoItemPresentation {
        icon: ICON_UNDO,
        label: String::from(LABEL),
        state: BurgerItemState::Idle,
        disabled,
        role: Some("menuitem"),
        onclick,
    }
}

impl ddd::Presentation for BurgerUndoItemPresentation {
    type Model = BurgerUndoItemModel;
}
