use dioxus::prelude::*;
use dioxus_primitives::dialog::{DialogContent, DialogRoot};

use crate::components::dialog_header::DialogHeader;
use crate::system_hotkeys::keycodes::KeyCodes;

static KEYBOARD_ROWS: &[&[(u32, &str)]] = &[
    &[
        (27, "Esc"),
        (112, "F1"),
        (113, "F2"),
        (114, "F3"),
        (115, "F4"),
        (116, "F5"),
        (117, "F6"),
        (118, "F7"),
        (119, "F8"),
        (120, "F9"),
        (121, "F10"),
        (122, "F11"),
        (123, "F12"),
    ],
    &[
        (192, "`"),
        (49, "1"),
        (50, "2"),
        (51, "3"),
        (52, "4"),
        (53, "5"),
        (54, "6"),
        (55, "7"),
        (56, "8"),
        (57, "9"),
        (48, "0"),
        (189, "-"),
        (187, "="),
    ],
    &[
        (81, "Q"),
        (87, "W"),
        (69, "E"),
        (82, "R"),
        (84, "T"),
        (89, "Y"),
        (85, "U"),
        (73, "I"),
        (79, "O"),
        (80, "P"),
        (219, "["),
        (221, "]"),
        (220, "\\"),
    ],
    &[
        (65, "A"),
        (83, "S"),
        (68, "D"),
        (70, "F"),
        (71, "G"),
        (72, "H"),
        (74, "J"),
        (75, "K"),
        (76, "L"),
        (186, ";"),
        (222, "'"),
    ],
    &[
        (90, "Z"),
        (88, "X"),
        (67, "C"),
        (86, "V"),
        (66, "B"),
        (78, "N"),
        (77, "M"),
        (188, ","),
        (190, "."),
        (191, "/"),
    ],
    &[
        (32, "Space"),
        (45, "Ins"),
        (46, "Del"),
        (36, "Home"),
        (35, "End"),
        (33, "PgUp"),
        (34, "PgDn"),
    ],
    &[(38, "↑"), (37, "←"), (40, "↓"), (39, "→")],
    // VK_XBUTTON1 (5, back) and VK_XBUTTON2 (6, forward) — confirmed against
    // a CustomKeys.txt exported by the in-game hotkey editor.
    &[(5, "Mouse4"), (6, "Mouse5")],
];

static NUMPAD_ROWS: &[&[(u32, &str)]] = &[
    &[(103, "Num7"), (104, "Num8"), (105, "Num9"), (111, "Num/")],
    &[(100, "Num4"), (101, "Num5"), (102, "Num6"), (106, "Num*")],
    &[(97, "Num1"), (98, "Num2"), (99, "Num3"), (109, "Num-")],
    &[(96, "Num0"), (110, "Num."), (107, "Num+")],
];

#[component]
pub(crate) fn SystemKeyPickerDialog(
    title: String,
    current_code: u32,
    open: bool,
    on_pick: EventHandler<u32>,
    on_close: EventHandler<()>,
) -> Element {
    let dialog_title = title.clone();
    rsx! {
        DialogRoot {
            class: "dialog-overlay",
            open,
            on_open_change: move |next_open: bool| {
                if !next_open {
                    on_close.call(());
                }
            },
            DialogContent { class: "dialog-shell wc3-dialog sys-key-picker-shell".to_string(),
                div {
                    class: "dialog-key-scope",
                    onkeydown: move |event| {
                        event.stop_propagation();
                        let key_val = event.data().key().to_string();
                        if key_val == "Escape" {
                            event.prevent_default();
                            on_close.call(());
                            return;
                        }
                        let code_val = event.data().code().to_string();
                        let Some(code) = KeyCodes::from_event(&key_val, &code_val) else {
                            return;
                        };
                        event.prevent_default();
                        on_pick.call(code);
                    },
                    DialogHeader {
                        title: dialog_title,
                        on_close: move |_| on_close.call(()),
                    }
                    div {
                        class: "wc3-dialog-body sys-key-picker-body",
                        tabindex: "-1",
                        autofocus: true,
                        div { class: "sys-key-picker-board",
                            div { class: "sys-key-picker-main",
                                for (row_idx, row) in KEYBOARD_ROWS.iter().enumerate() {
                                    div { key: "{row_idx}", class: "sys-key-picker-row",
                                        for (key_idx, entry) in row.iter().enumerate() {
                                            {
                                                let code = entry.0;
                                                let label = entry.1;
                                                let is_current = code == current_code;
                                                let is_wide = matches!(label, "Space" | "Mouse4" | "Mouse5");
                                                let cls = if is_current {
                                                    "sys-key-picker-key current"
                                                } else {
                                                    "sys-key-picker-key"
                                                };
                                                rsx! {
                                                    button {
                                                        key: "{key_idx}",
                                                        class: "{cls}",
                                                        r#type: "button",
                                                        "data-wide": if is_wide { "true" } else { "" },
                                                        onclick: move |_| on_pick.call(code),
                                                        "{label}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "sys-key-picker-numpad",
                                for (row_idx, row) in NUMPAD_ROWS.iter().enumerate() {
                                    div { key: "{row_idx}", class: "sys-key-picker-row",
                                        for (key_idx, entry) in row.iter().enumerate() {
                                            {
                                                let code = entry.0;
                                                let label = entry.1;
                                                let is_current = code == current_code;
                                                let cls = if is_current {
                                                    "sys-key-picker-key current"
                                                } else {
                                                    "sys-key-picker-key"
                                                };
                                                rsx! {
                                                    button {
                                                        key: "{key_idx}",
                                                        class: "{cls}",
                                                        r#type: "button",
                                                        onclick: move |_| on_pick.call(code),
                                                        "{label}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
