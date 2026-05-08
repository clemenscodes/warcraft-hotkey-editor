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
use crate::model::grid::{EditingCell, GridLayout};
use crate::services::customkeys::upload_status::UploadStatus;

use brand::AppHeaderBrand;
use burger::BurgerMenu;
use toolbar::HeaderToolbar;

const APP_HEADER_STYLES: Asset = asset!("/src/components/header/header.css");

// Shared by toolbar.rs and burger.rs via `super::`.
const TOOLBAR_BTN_CLASS: &str = "inline-flex items-center justify-center \
     [width:calc(5rem_*_var(--hdr-scale))] [height:calc(5rem_*_var(--hdr-scale))] p-0 \
     [background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)] \
     border border-[#6c5a1f] [border-radius:calc(12px_*_var(--hdr-scale))] \
     text-warcraft-text-secondary cursor-pointer \
     [transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease] \
     [@media(hover:hover)]:hover:border-warcraft-gold \
     [@media(hover:hover)]:hover:text-warcraft-gold \
     [@media(hover:hover)]:hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)] \
     [@media(hover:hover)]:hover:[box-shadow:0_0_12px_rgba(255,206,99,0.3)] \
     focus:outline-none \
     focus-visible:border-white focus-visible:text-white \
     focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)]";

const TOOLBAR_ICON_CLASS: &str = "flex items-center justify-center \
     [width:calc(2.2rem_*_var(--hdr-scale))] [height:calc(2.2rem_*_var(--hdr-scale))] \
     leading-none [&_svg]:block [&_svg]:w-full [&_svg]:h-full";

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
            AppHeaderBrand {}
            div {
                class: "hidden min-[1500px]:flex min-[1500px]:items-center min-[1500px]:justify-center",
                button {
                    class: "inline-flex items-center \
                            [gap:calc(1rem_*_var(--hdr-scale))] \
                            [height:calc(6rem_*_var(--hdr-scale))] \
                            [padding:0_calc(2rem_*_var(--hdr-scale))] \
                            [background:linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)] \
                            border border-warcraft-gold \
                            [border-radius:calc(12px_*_var(--hdr-scale))] \
                            text-warcraft-gold font-mono \
                            [font-size:calc(2rem_*_var(--hdr-scale))] \
                            tracking-[0.14em] font-medium cursor-pointer \
                            [box-shadow:0_0_22px_rgba(255,206,99,0.22)] \
                            [transition:background_0.12s_ease,box-shadow_0.12s_ease,transform_0.12s_ease] \
                            focus:outline-none \
                            [@media(hover:hover)]:hover:[background:linear-gradient(135deg,rgba(255,206,99,0.22)_0%,rgba(60,45,14,0.95)_100%)] \
                            [@media(hover:hover)]:hover:[box-shadow:0_0_26px_rgba(255,206,99,0.55),inset_0_0_14px_rgba(255,206,99,0.15)] \
                            focus-visible:border-white focus-visible:text-white \
                            focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_18px_rgba(255,255,255,0.55)]",
                    r#type: "button",
                    aria_label: "Edit global hotkey layout",
                    aria_haspopup: "dialog",
                    aria_expanded: "{layout_dialog_open()}",
                    onclick: move |_| {
                        let next = !*layout_dialog_open.read();
                        layout_dialog_open.set(next);
                    },
                    span {
                        class: "inline-flex \
                                [width:calc(2.2rem_*_var(--hdr-scale))] \
                                [height:calc(2.2rem_*_var(--hdr-scale))] \
                                [&_svg]:w-full [&_svg]:h-full",
                        aria_hidden: "true",
                        dangerous_inner_html: ICON_GRID,
                    }
                    span {
                        class: "font-friz-quadrata font-normal uppercase tracking-[0.12em] \
                                [text-shadow:1px_1px_0_rgba(0,0,0,0.6)]",
                        "GRID LAYOUT"
                    }
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
