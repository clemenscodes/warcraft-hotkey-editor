mod brand;
mod burger;
mod toolbar;

use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use warcraft_keybinds::CustomKeys;

use crate::components::dialog_stack::nested_picker_dialog_is_present;
use crate::components::icons::ICON_GRID;
use crate::components::layout_editor::LayoutEditor;
use crate::components::templates_dialog::TemplatesDialog;
use crate::customkeys::upload_status::UploadStatus;
use crate::grid_layout::{EditingCell, GridLayout};

use brand::AppHeaderBrand;
use burger::BurgerMenu;
use toolbar::HeaderToolbar;

const HEADER_GOLD_DECORATION: Asset = asset!("/assets/webui/common/header-decoration-gold.png");

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
                    header { class: "wc3-dialog-header",
                        img {
                            class: "wc3-header-decoration",
                            src: "{HEADER_GOLD_DECORATION}",
                            alt: "",
                            aria_hidden: "true",
                        }
                        h2 { class: "wc3-dialog-title", "Global Hotkey Layout" }
                        img {
                            class: "wc3-header-decoration wc3-header-decoration-mirrored",
                            src: "{HEADER_GOLD_DECORATION}",
                            alt: "",
                            aria_hidden: "true",
                        }
                        button {
                            class: "wc3-close-button",
                            r#type: "button",
                            aria_label: "Close",
                            onclick: move |_| layout_dialog_open.set(false),
                            "\u{2715}"
                        }
                    }
                    div { class: "wc3-dialog-body layout-editor-body",
                        div { class: "layout-editor-explainer",
                            p { class: "layout-editor-explainer-line",
                                "Define a hotkey letter for each button position."
                            }
                            p { class: "layout-editor-explainer-line",
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
