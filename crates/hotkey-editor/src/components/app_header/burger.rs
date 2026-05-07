use dioxus::prelude::*;
use warcraft_keybinds::CustomKeys;

use crate::components::download_info_dialog::DownloadInfoDialog;
use crate::components::icons::{
    ICON_BURGER, ICON_COG, ICON_DOWNLOAD, ICON_GRID, ICON_PREVIEW, ICON_TEMPLATES, ICON_UPLOAD,
};
use crate::components::upload_info_dialog::UploadInfoDialog;
use crate::customkeys::blob_download::BlobDownload;

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
            class: "app-header-burger-btn toolbar-icon-button",
            r#type: "button",
            aria_label: "Open menu",
            aria_expanded: "{burger_open()}",
            aria_controls: "burger-drawer",
            onclick: move |_| {
                let next = !*burger_open.read();
                burger_open.set(next);
            },
            span {
                class: "toolbar-icon",
                aria_hidden: "true",
                dangerous_inner_html: ICON_BURGER,
            }
        }
        if burger_open() {
            div {
                class: "burger-backdrop",
                role: "button",
                aria_label: "Close menu",
                tabindex: "-1",
                onclick: move |_| burger_open.set(false),
            }
            div {
                id: "burger-drawer",
                class: "burger-drawer",
                role: "navigation",
                aria_label: "Menu",
                div { class: "burger-drawer-header",
                    button {
                        class: "burger-close-btn",
                        r#type: "button",
                        aria_label: "Close menu",
                        onclick: move |_| burger_open.set(false),
                        "\u{2715}"
                    }
                }
                div { class: "burger-drawer-body",
                    button {
                        class: "burger-menu-item burger-grid-layout-item",
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
                            class: "burger-menu-item-icon",
                            aria_hidden: "true",
                            dangerous_inner_html: ICON_GRID,
                        }
                        span { class: "burger-menu-item-label", "Grid Layout" }
                    }
                    div {
                        class: "burger-menu-list",
                        role: "menu",
                        aria_label: "File actions",
                        button {
                            class: "burger-menu-item",
                            r#type: "button",
                            role: "menuitem",
                            onclick: move |_| {
                                burger_upload_info_open.set(true);
                                burger_open.set(false);
                            },
                            span {
                                class: "burger-menu-item-icon",
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_UPLOAD,
                            }
                            span { class: "burger-menu-item-label", "Upload" }
                        }
                        button {
                            class: "burger-menu-item",
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
                                class: "burger-menu-item-icon",
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_TEMPLATES,
                            }
                            span { class: "burger-menu-item-label", "Browse Templates" }
                        }
                        button {
                            class: if system_hotkeys_open() { "burger-menu-item active" } else { "burger-menu-item" },
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
                                class: "burger-menu-item-icon",
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_COG,
                            }
                            span { class: "burger-menu-item-label", "System Hotkeys" }
                        }
                        button {
                            class: if preview_active { "burger-menu-item active" } else { "burger-menu-item" },
                            r#type: "button",
                            role: "menuitem",
                            aria_pressed: "{preview_active}",
                            onclick: move |_| {
                                let next = !*preview_open.read();
                                preview_open.set(next);
                                burger_open.set(false);
                            },
                            span {
                                class: "burger-menu-item-icon",
                                aria_hidden: "true",
                                dangerous_inner_html: ICON_PREVIEW,
                            }
                            span {
                                class: "burger-menu-item-label",
                                if preview_active { "Hide Preview" } else { "Preview" }
                            }
                        }
                        if has_loaded_file {
                            button {
                                class: "burger-menu-item",
                                r#type: "button",
                                role: "menuitem",
                                onclick: move |_| {
                                    burger_download_info_open.set(true);
                                    burger_open.set(false);
                                },
                                span {
                                    class: "burger-menu-item-icon",
                                    aria_hidden: "true",
                                    dangerous_inner_html: ICON_DOWNLOAD,
                                }
                                span { class: "burger-menu-item-label", "Download" }
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
