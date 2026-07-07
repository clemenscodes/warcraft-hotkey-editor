use super::state::HotkeyBadgeState;
use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "min-w-[24cqi]",
        "h-[24cqi]",
        "px-[5cqi]",
        "rounded-[5cqi]",
        "border",
        "text-[17cqi]/[1]",
        "font-normal",
        "pointer-events-none",
        "text-shadow-drop",
    ],
}

states! {
    HotkeyBadgeState,
    Normal => tw![
        "bg-warcraft-shadow/78",
        "border-warcraft-gold/55",
        "text-warcraft-gold",
    ],
    Passive => tw![
        "bg-warcraft-bg-mid",
        "border-warcraft-text-faint",
        "text-warcraft-text-secondary",
    ],
    Conflict => tw![
        "bg-race-orc-strong/85",
        "border-warcraft-danger",
        "text-warcraft-danger",
    ],
}
