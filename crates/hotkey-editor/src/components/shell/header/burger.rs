use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::dialogs::download_info_dialog::DownloadInfoDialog;
use crate::components::dialogs::upload_info_dialog::UploadInfoDialog;
use crate::components::shared::icons::{
    ICON_BURGER, ICON_COG, ICON_DOWNLOAD, ICON_GRID, ICON_PREVIEW, ICON_TEMPLATES, ICON_UPLOAD,
};
use crate::services::files::download::BlobDownload;

const BURGER_MENU_ITEM_CLASS: &str = "flex items-center gap-[0.85rem] w-full min-h-12 py-[0.65rem] px-[0.9rem] \
     [background:linear-gradient(180deg,rgba(40,30,8,0.55)_0%,rgba(15,12,4,0.55)_100%)] \
     border border-[#6c5a1f] rounded-[10px] text-warcraft-text-secondary \
     font-friz-quadrata text-[1rem] tracking-[0.05em] text-left cursor-pointer \
     [transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease] \
     [@media(hover:hover)]:hover:border-warcraft-gold \
     [@media(hover:hover)]:hover:text-warcraft-gold \
     [@media(hover:hover)]:hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)] \
     [@media(hover:hover)]:hover:[box-shadow:0_0_12px_rgba(255,206,99,0.3)] \
     focus:outline-none \
     focus-visible:border-white focus-visible:text-white \
     focus-visible:[box-shadow:0_0_0_2px_#fff,0_0_16px_rgba(255,255,255,0.55)]";

const BURGER_MENU_ITEM_ACTIVE_CLASS: &str = "flex items-center gap-[0.85rem] w-full min-h-12 py-[0.65rem] px-[0.9rem] \
     [background:linear-gradient(180deg,rgba(255,206,99,0.22)_0%,rgba(40,30,8,0.6)_100%)] \
     border border-warcraft-gold rounded-[10px] text-warcraft-gold \
     font-friz-quadrata text-[1rem] tracking-[0.05em] text-left cursor-pointer \
     [box-shadow:inset_0_0_0_1px_rgba(255,206,99,0.3),0_0_14px_rgba(255,206,99,0.22)] \
     [transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease] \
     [@media(hover:hover)]:hover:border-warcraft-gold \
     [@media(hover:hover)]:hover:text-warcraft-gold \
     [@media(hover:hover)]:hover:[background:linear-gradient(180deg,rgba(255,206,99,0.18)_0%,rgba(40,30,8,0.55)_100%)] \
     [@media(hover:hover)]:hover:[box-shadow:0_0_12px_rgba(255,206,99,0.3)] \
     focus:outline-none \
     focus-visible:border-white focus-visible:text-white \
     focus-visible:[box-shadow:0_0_0_2px_#fff,0_0_16px_rgba(255,255,255,0.55)]";

const BURGER_MENU_ITEM_ICON_CLASS: &str = "inline-flex items-center justify-center w-6 h-6 shrink-0 \
     text-inherit [&_svg]:w-full [&_svg]:h-full";

const BURGER_MENU_ITEM_LABEL_CLASS: &str =
    "flex-1 leading-[1.25] [text-shadow:1px_1px_0_rgba(0,0,0,0.6)]";

#[component]
pub(crate) fn BurgerMenu(
    loaded_keys: Signal<Option<CustomKeys>>,
    preview_open: Signal<bool>,
    layout_dialog_open: Signal<bool>,
    templates_dialog_open: Signal<bool>,
    mut system_hotkeys_open: Signal<bool>,
) -> Element {
    let mut burger_open = use_signal::<bool>(|| false);
    let mut burger_upload_info_open = use_signal::<bool>(|| false);
    let mut burger_download_info_open = use_signal::<bool>(|| false);
    let mut preview_open = preview_open;
    let has_loaded_file = loaded_keys.read().is_some();
    let preview_active = preview_open();

    rsx! {
        button {
            class: "inline-flex items-center justify-center \
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
                    focus-visible:[box-shadow:0_0_0_3px_#fff,0_0_16px_rgba(255,255,255,0.55)] \
                    shrink-0 self-center min-[1500px]:hidden \
                    max-[1099px]:w-[44px] max-[1099px]:h-[44px] \
                    max-[1099px]:min-w-[44px] max-[1099px]:min-h-[44px]",
            r#type: "button",
            aria_label: "Open menu",
            aria_expanded: "{burger_open()}",
            aria_controls: "burger-drawer",
            onclick: move |_| {
                let next = !*burger_open.read();
                burger_open.set(next);
            },
            span {
                class: "flex items-center justify-center \
                        [width:calc(2.2rem_*_var(--hdr-scale))] [height:calc(2.2rem_*_var(--hdr-scale))] \
                        leading-none [&_svg]:block [&_svg]:w-full [&_svg]:h-full \
                        max-[1099px]:[width:1.4rem] max-[1099px]:[height:1.4rem]",
                aria_hidden: "true",
                dangerous_inner_html: ICON_BURGER,
            }
        }
        if burger_open() {
            div {
                class: "fixed inset-0 z-[70] bg-[rgba(0,0,0,0.65)] cursor-pointer border-none p-0",
                role: "button",
                aria_label: "Close menu",
                tabindex: "-1",
                onclick: move |_| burger_open.set(false),
            }
            div {
                id: "burger-drawer",
                class: "fixed top-0 right-0 h-dvh max-h-dvh z-[71] \
                        [width:min(85vw,320px)] \
                        [background:linear-gradient(170deg,#0c1d30_0%,#070e1c_100%)] \
                        border-l border-l-[rgba(255,206,99,0.3)] \
                        [box-shadow:-6px_0_40px_rgba(0,0,0,0.85)] \
                        flex flex-col \
                        [animation:burger-slide-in_0.22s_cubic-bezier(0.16,1,0.3,1)] \
                        [padding-top:env(safe-area-inset-top)] \
                        [padding-bottom:env(safe-area-inset-bottom)]",
                role: "navigation",
                aria_label: "Menu",
                div {
                    class: "flex items-center justify-end py-3 px-4 \
                            border-b border-b-[rgba(255,206,99,0.12)] shrink-0",
                    button {
                        class: "inline-flex items-center justify-center w-9 h-9 p-0 \
                                bg-transparent border border-[rgba(255,206,99,0.3)] \
                                rounded-[8px] text-[rgba(255,206,99,0.7)] text-[0.9rem] \
                                cursor-pointer \
                                [transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease] \
                                [@media(hover:hover)]:hover:border-warcraft-gold \
                                [@media(hover:hover)]:hover:text-warcraft-gold \
                                [@media(hover:hover)]:hover:bg-[rgba(255,206,99,0.08)] \
                                focus:outline-none \
                                focus-visible:border-white focus-visible:text-white \
                                focus-visible:[box-shadow:0_0_0_2px_#fff]",
                        r#type: "button",
                        aria_label: "Close menu",
                        onclick: move |_| burger_open.set(false),
                        "\u{2715}"
                    }
                }
                div {
                    class: "flex-1 flex flex-col gap-5 py-6 px-5 overflow-y-auto \
                            max-[1099px]:gap-4 max-[1099px]:pt-5 max-[1099px]:px-4 \
                            max-[1099px]:[padding-bottom:max(1.25rem,env(safe-area-inset-bottom))]",
                    button {
                        class: "flex items-center gap-[0.85rem] w-full min-h-12 \
                                py-[0.65rem] px-[0.9rem] \
                                [background:linear-gradient(135deg,rgba(40,30,8,0.85)_0%,rgba(15,12,4,0.85)_100%)] \
                                border border-warcraft-gold rounded-[10px] \
                                text-warcraft-gold font-friz-quadrata text-[1rem] \
                                tracking-[0.05em] text-left cursor-pointer \
                                [box-shadow:0_0_22px_rgba(255,206,99,0.22)] \
                                [transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease] \
                                [@media(hover:hover)]:hover:[background:linear-gradient(135deg,rgba(255,206,99,0.22)_0%,rgba(60,45,14,0.95)_100%)] \
                                [@media(hover:hover)]:hover:[box-shadow:0_0_26px_rgba(255,206,99,0.55),inset_0_0_14px_rgba(255,206,99,0.15)] \
                                focus:outline-none \
                                focus-visible:border-white focus-visible:text-white \
                                focus-visible:[box-shadow:0_0_0_2px_#fff,0_0_16px_rgba(255,255,255,0.55)]",
                        r#type: "button",
                        aria_label: "Edit global hotkey layout",
                        aria_haspopup: "dialog",
                        aria_expanded: "{layout_dialog_open()}",
                        onclick: move |_| {
                            let next = !*layout_dialog_open.read();
                            layout_dialog_open.set(next);
                            burger_open.set(false);
                        },
                        span {
                            class: BURGER_MENU_ITEM_ICON_CLASS,
                            aria_hidden: "true",
                            dangerous_inner_html: ICON_GRID,
                        }
                        span { class: BURGER_MENU_ITEM_LABEL_CLASS, "Grid Layout" }
                    }
                    div {
                        class: "flex flex-col items-stretch gap-[0.4rem] \
                                mt-1 pt-[0.85rem] \
                                border-t border-t-[rgba(255,206,99,0.12)]",
                        role: "menu",
                        aria_label: "File actions",
                        button {
                            class: BURGER_MENU_ITEM_CLASS,
                            r#type: "button",
                            role: "menuitem",
                            onclick: move |_| {
                                burger_upload_info_open.set(true);
                                burger_open.set(false);
                            },
                            span {
                                class: BURGER_MENU_ITEM_ICON_CLASS,
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_UPLOAD,
                            }
                            span { class: BURGER_MENU_ITEM_LABEL_CLASS, "Upload" }
                        }
                        button {
                            class: BURGER_MENU_ITEM_CLASS,
                            r#type: "button",
                            role: "menuitem",
                            aria_haspopup: "dialog",
                            aria_expanded: "{templates_dialog_open()}",
                            onclick: move |_| {
                                let next = !*templates_dialog_open.read();
                                templates_dialog_open.set(next);
                                burger_open.set(false);
                            },
                            span {
                                class: BURGER_MENU_ITEM_ICON_CLASS,
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_TEMPLATES,
                            }
                            span { class: BURGER_MENU_ITEM_LABEL_CLASS, "Browse Templates" }
                        }
                        button {
                            class: if system_hotkeys_open() { BURGER_MENU_ITEM_ACTIVE_CLASS } else { BURGER_MENU_ITEM_CLASS },
                            r#type: "button",
                            role: "menuitem",
                            aria_haspopup: "dialog",
                            aria_expanded: "{system_hotkeys_open()}",
                            onclick: move |_| {
                                let next = !*system_hotkeys_open.read();
                                system_hotkeys_open.set(next);
                                burger_open.set(false);
                            },
                            span {
                                class: BURGER_MENU_ITEM_ICON_CLASS,
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_COG,
                            }
                            span { class: BURGER_MENU_ITEM_LABEL_CLASS, "System Hotkeys" }
                        }
                        button {
                            class: if preview_active { BURGER_MENU_ITEM_ACTIVE_CLASS } else { BURGER_MENU_ITEM_CLASS },
                            r#type: "button",
                            role: "menuitem",
                            aria_pressed: "{preview_active}",
                            onclick: move |_| {
                                let next = !*preview_open.read();
                                preview_open.set(next);
                                burger_open.set(false);
                            },
                            span {
                                class: BURGER_MENU_ITEM_ICON_CLASS,
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_PREVIEW,
                            }
                            span {
                                class: BURGER_MENU_ITEM_LABEL_CLASS,
                                if preview_active { "Hide Preview" } else { "Preview" }
                            }
                        }
                        if has_loaded_file {
                            button {
                                class: BURGER_MENU_ITEM_CLASS,
                                r#type: "button",
                                role: "menuitem",
                                onclick: move |_| {
                                    burger_download_info_open.set(true);
                                    burger_open.set(false);
                                },
                                span {
                                    class: BURGER_MENU_ITEM_ICON_CLASS,
                                    aria_hidden: "true",
                                    dangerous_inner_html: ICON_DOWNLOAD,
                                }
                                span { class: BURGER_MENU_ITEM_LABEL_CLASS, "Download" }
                            }
                        }
                    }
                }
            }
        }
        UploadInfoDialog { open: burger_upload_info_open }
        if has_loaded_file {
            DownloadInfoDialog {
                open: burger_download_info_open,
                on_confirm: move |_| {
                    let serialized = {
                        let read_guard = loaded_keys.read();
                        let Some(file) = read_guard.as_ref() else { return };
                        file.normalize().to_string()
                    };
                    BlobDownload::trigger("CustomKeys.txt", &serialized);
                },
            }
        }
    }
}
