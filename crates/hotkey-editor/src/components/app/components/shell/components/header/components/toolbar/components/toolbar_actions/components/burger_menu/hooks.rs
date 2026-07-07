use dioxus::prelude::*;

use super::components::burger_drawer::BurgerDrawerProps;
use super::logic::{BurgerActionHandlers, MenuRowBuilder, RowDynamics};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::info_dialogs::upload_info_dialog::UploadInfoDialogProps;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::context::use_view_navigation;
use crate::services::overlay_state::context::use_overlay_state;
use crate::services::undo::context::use_undo_history;

/// The already-shaped controller state the body renders: the drawer open flags,
/// the toggle and download handlers, and the fully-built drawer props (primary
/// row plus grouped action rows, each with its handler and state).
pub struct BurgerMenuView {
    pub burger_open: Signal<bool>,
    pub upload_info_open: Signal<bool>,
    pub download_info_open: Signal<bool>,
    pub toggle: EventHandler<MouseEvent>,
    pub drawer: BurgerDrawerProps,
}

/// The drawer's open state and the body-scroll lock it drives: owns the
/// `burger_open` signal, the effect that pins the page while the drawer is open,
/// and the toggle/close handlers.
pub(super) struct BurgerOpen {
    pub(super) burger_open: Signal<bool>,
    pub(super) toggle: EventHandler<MouseEvent>,
    pub(super) close: EventHandler<MouseEvent>,
}

/// The action rows' live state, their own info-dialog open signals, and every
/// row handler — reading the undo history, navigation, and overlay dialogs once
/// and wiring each row to close the drawer plus perform its action.
pub(super) struct BurgerActions {
    pub(super) upload_info_open: Signal<bool>,
    pub(super) download_info_open: Signal<bool>,
    pub(super) dynamics: RowDynamics,
    pub(super) handlers: BurgerActionHandlers,
}

fn use_burger_open() -> BurgerOpen {
    let mut burger_open = use_signal::<bool>(|| false);
    use_effect(move || {
        let is_open = burger_open();
        let Some(body) = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.body())
        else {
            return;
        };
        let style = body.style();
        if is_open {
            let _ = style.set_property("overflow", "hidden");
            let _ = style.set_property("overscroll-behavior", "contain");
        } else {
            let _ = style.remove_property("overflow");
            let _ = style.remove_property("overscroll-behavior");
        }
    });
    let toggle = EventHandler::new(move |_event: MouseEvent| {
        let next = !*burger_open.read();
        burger_open.set(next);
    });
    let close = EventHandler::new(move |_event: MouseEvent| burger_open.set(false));
    BurgerOpen {
        burger_open,
        toggle,
        close,
    }
}

fn use_burger_actions(burger_open: Signal<bool>) -> BurgerActions {
    let navigation = use_view_navigation();
    let overlay = use_overlay_state();
    let history = use_undo_history();
    let mut burger_open = burger_open;
    let mut system_hotkeys_open = overlay.system_hotkeys_open();
    let mut help_open = overlay.help_open();
    let mut layout_dialog_open = overlay.layout_dialog_open();
    let mut templates_dialog_open = overlay.templates_dialog_open();
    let mut preview_open = overlay.preview_open();
    let mut upload_info_open = use_signal::<bool>(|| false);
    let mut download_info_open = use_signal::<bool>(|| false);

    let preview_active = preview_open();
    let system_hotkeys_active = system_hotkeys_open();
    let layout_expanded = layout_dialog_open();
    let templates_expanded = templates_dialog_open();
    let help_expanded = help_open();
    let can_undo = history.can_undo();
    let can_redo = history.can_redo();

    let toggle_layout = EventHandler::new(move |_event: MouseEvent| {
        let next = !*layout_dialog_open.read();
        layout_dialog_open.set(next);
        burger_open.set(false);
    });
    let open_upload = EventHandler::new(move |_event: MouseEvent| {
        upload_info_open.set(true);
        burger_open.set(false);
    });
    let toggle_templates = EventHandler::new(move |_event: MouseEvent| {
        let next = !*templates_dialog_open.read();
        templates_dialog_open.set(next);
        burger_open.set(false);
    });
    let toggle_system_hotkeys = EventHandler::new(move |_event: MouseEvent| {
        let next = !*system_hotkeys_open.read();
        system_hotkeys_open.set(next);
        burger_open.set(false);
    });
    let open_help = EventHandler::new(move |_event: MouseEvent| {
        help_open.set(true);
        burger_open.set(false);
    });
    let toggle_preview = EventHandler::new(move |_event: MouseEvent| {
        let next = !*preview_open.read();
        preview_open.set(next);
        burger_open.set(false);
    });
    let open_download = EventHandler::new(move |_event: MouseEvent| {
        download_info_open.set(true);
        burger_open.set(false);
    });
    let open_resolve = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        navigation.apply(AppView::Resolve);
    });
    let trigger_undo = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        history.undo();
    });
    let trigger_redo = EventHandler::new(move |_event: MouseEvent| {
        burger_open.set(false);
        history.redo();
    });

    let dynamics = RowDynamics {
        can_undo,
        can_redo,
        system_hotkeys_active,
        preview_active,
        layout_expanded,
        templates_expanded,
        help_expanded,
    };
    let handlers = BurgerActionHandlers {
        toggle_layout,
        trigger_undo,
        trigger_redo,
        open_upload,
        toggle_templates,
        toggle_system_hotkeys,
        toggle_preview,
        open_resolve,
        open_download,
        open_help,
    };
    BurgerActions {
        upload_info_open,
        download_info_open,
        dynamics,
        handlers,
    }
}

/// The composed hook: owns the drawer's local open state, reads the undo history
/// and the live config, and wires every row's handler and state. The body only
/// names the result.
pub fn use_burger_menu() -> BurgerMenuView {
    let drawer = use_burger_open();
    let actions = use_burger_actions(drawer.burger_open);
    let builder = MenuRowBuilder {
        dynamics: actions.dynamics,
        handlers: actions.handlers,
    };
    let layout = builder.layout();
    let items = builder.items();
    let on_close = drawer.close;
    let drawer_props = BurgerDrawerProps {
        on_close,
        layout,
        items,
    };
    BurgerMenuView {
        burger_open: drawer.burger_open,
        upload_info_open: actions.upload_info_open,
        download_info_open: actions.download_info_open,
        toggle: drawer.toggle,
        drawer: drawer_props,
    }
}

impl From<&BurgerMenuView> for UploadInfoDialogProps {
    fn from(view: &BurgerMenuView) -> Self {
        let open = view.upload_info_open;
        Self { open }
    }
}
