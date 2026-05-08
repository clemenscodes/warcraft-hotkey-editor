mod brand;
mod burger;
mod toolbar;

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use warcraft_keybinds::CustomKeys;

use crate::components::dialog_header::DialogHeader;
use crate::components::dialog_stack::nested_picker_dialog_is_present;
use crate::components::icons::ICON_GRID;
use crate::components::layout_editor::LayoutEditor;
use crate::components::templates_dialog::TemplatesDialog;
use crate::customkeys::upload_status::UploadStatus;
use crate::grid_layout::{EditingCell, GridLayout};

use brand::AppHeaderBrand;
use burger::BurgerMenu;
use toolbar::HeaderToolbar;

#[component]
pub(crate) fn AppHeader(
    loaded_keys: Signal<Option<CustomKeys>>,
    upload_status: Signal<UploadStatus>,
    preview_open: Signal<bool>,
    grid_layout: Signal<GridLayout>,
    editing_layout_cell: Signal<Option<EditingCell>>,
    dragging_layout_cell: Signal<Option<EditingCell>>,
    mut system_hotkeys_open: Signal<bool>,
) -> Element {
    let mut layout_dialog_open = use_signal::<bool>(|| false);
    let templates_dialog_open = use_signal::<bool>(|| false);

    rsx! {
        header { class: "app-header",
            AppHeaderBrand {}
            div { class: "app-header-center",
                button {
                    class: "layout-pill",
                    r#type: "button",
                    aria_label: "Edit global hotkey layout",
                    aria_haspopup: "dialog",
                    aria_expanded: "{layout_dialog_open()}",
                    onclick: move |_| {
                        let next = !*layout_dialog_open.read();
                        layout_dialog_open.set(next);
                    },
                    span {
                        class: "layout-pill-icon",
                        aria_hidden: "true",
                        dangerous_inner_html: ICON_GRID,
                    }
                    span { class: "layout-pill-letters", "GRID LAYOUT" }
                }
            }
            HeaderToolbar {
                loaded_keys,
                upload_status,
                preview_open,
                templates_dialog_open,
                system_hotkeys_open,
            }
            BurgerMenu {
                loaded_keys,
                preview_open,
                layout_dialog_open,
                templates_dialog_open,
                system_hotkeys_open,
            }
        }
        if templates_dialog_open() {
            TemplatesDialog { loaded_keys, upload_status, templates_dialog_open }
        }
        if layout_dialog_open() {
            DialogRoot {
                class: "dialog-overlay",
                open: layout_dialog_open(),
                on_open_change: move |is_open: bool| {
                    if !is_open && nested_picker_dialog_is_present() {
                        return;
                    }
                    layout_dialog_open.set(is_open);
                },
                DialogContent { class: "dialog-shell wc3-dialog layout-editor-shell".to_string(),
                    DialogHeader {
                        title: "Global Hotkey Layout".to_string(),
                        on_close: move |_| layout_dialog_open.set(false),
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
                            loaded_keys,
                        }
                    }
                }
            }
        }
    }
}
