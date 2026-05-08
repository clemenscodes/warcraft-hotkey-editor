use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};
use warcraft_keybinds::CustomKeys;

use crate::components::dialog_header::DialogHeader;

const PREVIEW_STYLES: Asset = asset!("/src/components/preview_dialog/preview_dialog.css");

#[component]
pub(crate) fn PreviewDialog(
    loaded_keys: Signal<Option<CustomKeys>>,
    mut preview_open: Signal<bool>,
) -> Element {
    let preview_text = use_memo(move || {
        let read_guard = loaded_keys.read();
        match read_guard.as_ref() {
            Some(file) => file.normalize().to_string(),
            None => String::new(),
        }
    });
    rsx! {
        document::Stylesheet { href: PREVIEW_STYLES }
        DialogRoot {
            class: "dialog-overlay",
            open: preview_open(),
            on_open_change: move |is_open| preview_open.set(is_open),
            DialogContent { class: "dialog-shell wc3-dialog preview-dialog".to_string(),
                DialogHeader {
                    title: "Preview".to_string(),
                    on_close: move |_| preview_open.set(false),
                }
                div { class: "wc3-dialog-body flex flex-col gap-6",
                    textarea {
                        class: "preview-textarea w-full flex-1 min-h-80 px-8 py-6 \
                            rounded-md border border-warcraft-blue \
                            bg-[rgba(8,18,35,0.85)] text-warcraft-text-primary \
                            font-mono text-[1.8rem] leading-[1.45] \
                            whitespace-pre overflow-auto resize-y \
                            [scrollbar-width:thin] \
                            [scrollbar-color:rgba(255,206,99,0.45)_transparent] \
                            focus:outline-none focus:border-warcraft-gold \
                            focus:[box-shadow:0_0_8px_rgba(255,206,99,0.4)] \
                            max-[480px]:text-[1.4rem]",
                        readonly: true,
                        spellcheck: false,
                        wrap: "off",
                        value: "{preview_text.read()}",
                    }
                }
            }
        }
    }
}
