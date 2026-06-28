mod brand;
mod burger;
mod collisions_button;
mod toolbar;

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use warcraft_api::Race;
use warcraft_database::UnitMode;
use warcraft_keybinds::CustomKeys;

use crate::components::actions::grid_layout_button::GridLayoutButton;
use crate::components::dialogs::dialog_header::DialogHeader;
use crate::components::dialogs::dialog_stack::nested_picker_dialog_is_present;
use crate::components::dialogs::layout_editor::LayoutEditor;
use crate::components::dialogs::templates_dialog::TemplatesDialog;
use crate::services::customkeys::upload_status::UploadStatus;
use crate::services::navigation::app_view::AppView;
use crate::services::navigation::view_navigation::ViewNavigationContext;
use warcraft_keybinds::{GridCoordinate, GridLayout};

pub use brand::HeaderBrand;
pub use burger::BurgerMenu;
pub use collisions_button::CollisionsButton;
pub use toolbar::HeaderToolbar;

const APP_HEADER_STYLES: Asset = asset!("/src/components/shell/header/header.css");

#[derive(Props, Clone, PartialEq)]
pub struct HeaderProps {
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub upload_status: Signal<UploadStatus>,
    pub preview_open: Signal<bool>,
    pub grid_layout: Signal<GridLayout>,
    pub editing_layout_cell: Signal<Option<GridCoordinate>>,
    pub dragging_layout_cell: Signal<Option<GridCoordinate>>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub system_hotkeys_open: Signal<bool>,
    pub help_open: Signal<bool>,
    pub current_view: Signal<AppView>,
    pub active_race: Signal<Race>,
    pub unit_mode: Signal<UnitMode>,
    pub selected_unit_id: Signal<Option<String>>,
    pub search_query: Signal<String>,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    let loaded_keys = props.loaded_keys;
    let upload_status = props.upload_status;
    let preview_open = props.preview_open;
    let grid_layout = props.grid_layout;
    let editing_layout_cell = props.editing_layout_cell;
    let dragging_layout_cell = props.dragging_layout_cell;
    let update_hotkeys_on_move = props.update_hotkeys_on_move;
    let system_hotkeys_open = props.system_hotkeys_open;
    let help_open = props.help_open;
    let navigation = ViewNavigationContext {
        current_view: props.current_view,
        active_race: props.active_race,
        unit_mode: props.unit_mode,
        selected_unit_id: props.selected_unit_id,
        search_query: props.search_query,
    };
    let mut layout_dialog_open = use_signal::<bool>(|| false);
    let templates_dialog_open = use_signal::<bool>(|| false);
    let handle_layout_open_change = move |is_open: bool| {
        if !is_open && nested_picker_dialog_is_present() {
            return;
        }
        layout_dialog_open.set(is_open);
    };
    let close_layout_dialog = move |_| layout_dialog_open.set(false);

    rsx! {
        document::Stylesheet { href: APP_HEADER_STYLES }
        header {
            class: "relative z-50 flex flex-row items-center flex-none \
                    [padding:calc(1.5rem_*_var(--hdr-scale))_calc(1rem_*_var(--hdr-scale))] \
                    border-b border-b-[rgba(255,206,99,0.4)] \
                    [box-shadow:0_1px_0_rgba(0,0,0,0.7),0_2px_0_rgba(255,206,99,0.1)] \
                    max-[1099px]:sticky max-[1099px]:top-0 max-[1099px]:z-[60] \
                    max-[1099px]:[background:linear-gradient(180deg,rgba(8,14,30,0.98)_0%,rgba(8,14,30,0.96)_100%)] \
                    max-[1099px]:[padding-top:max(0.5rem,env(safe-area-inset-top))] \
                    max-[1099px]:pb-2 max-[1099px]:pl-2 max-[1099px]:pr-2 \
                    max-[1099px]:border-b-[rgba(255,206,99,0.3)] \
                    max-[1099px]:min-h-14 max-[1099px]:max-w-full max-[1099px]:w-full \
                    min-[1500px]:grid \
                    min-[1500px]:[grid-template-columns:minmax(0,1fr)_auto_minmax(0,1fr)] \
                    min-[1500px]:[gap:calc(1.5rem_*_var(--hdr-scale))] \
                    min-[1500px]:[padding:0_0_calc(1.75rem_*_var(--hdr-scale))_0]",
            HeaderBrand { navigation }
            div {
                class: "hidden min-[1500px]:flex min-[1500px]:items-center min-[1500px]:justify-center",
                GridLayoutButton { layout_dialog_open }
            }
            div {
                class: "flex flex-row items-center justify-end \
                        [gap:calc(0.65rem_*_var(--hdr-scale))] min-w-0 \
                        max-[1099px]:gap-2",
                CollisionsButton { loaded_keys, grid_layout, navigation }
                HeaderToolbar {
                    loaded_keys,
                    upload_status,
                    preview_open,
                    templates_dialog_open,
                    system_hotkeys_open,
                    help_open,
                    navigation,
                }
                BurgerMenu {
                    loaded_keys,
                    preview_open,
                    layout_dialog_open,
                    templates_dialog_open,
                    system_hotkeys_open,
                    help_open,
                    navigation,
                }
            }
        }
        if templates_dialog_open() {
            TemplatesDialog { loaded_keys, upload_status, templates_dialog_open }
        }
        if layout_dialog_open() {
            DialogRoot {
                class: "dialog-overlay",
                open: layout_dialog_open(),
                on_open_change: handle_layout_open_change,
                DialogContent { class: "dialog-shell wc3-dialog layout-editor-shell".to_string(),
                    DialogHeader {
                        title: "Global Hotkey Layout".to_string(),
                        on_close: close_layout_dialog,
                    }
                    div { class: "wc3-dialog-body flex flex-col items-center justify-center gap-[4rem] pt-[4rem] pb-[4rem] max-[1099px]:[flex:1_1_0] max-[1099px]:min-h-0 max-[1099px]:overflow-y-auto max-[1099px]:[-webkit-overflow-scrolling:touch] max-[1099px]:[overscroll-behavior:contain] max-[1099px]:justify-start max-[1099px]:gap-[20px] max-[1099px]:pt-[20px] max-[1099px]:pb-[20px]",
                        div { class: "flex flex-col items-center gap-[0.7rem] m-0 text-center [text-shadow:1px_1px_0_#000]",
                            p { class: "m-0 font-friz-quadrata uppercase tracking-[0.1em] text-[rgba(255,206,99,0.85)] text-[2.1rem] leading-[1.35] max-[1099px]:text-[clamp(13px,3.5vw,16px)] max-[1099px]:tracking-[0.05em]",
                                "Define a hotkey letter for each button position."
                            }
                            p { class: "m-0 font-friz-quadrata uppercase tracking-[0.1em] text-[rgba(255,206,99,0.85)] text-[2.1rem] leading-[1.35] max-[1099px]:text-[clamp(13px,3.5vw,16px)] max-[1099px]:tracking-[0.05em]",
                                "Click apply to rewrite every ability hotkey to match this grid layout."
                            }
                        }
                        LayoutEditor {
                            grid_layout,
                            editing_layout_cell,
                            dragging_layout_cell,
                            update_hotkeys_on_move,
                            loaded_keys,
                            layout_dialog_open,
                        }
                    }
                }
            }
        }
    }
}
