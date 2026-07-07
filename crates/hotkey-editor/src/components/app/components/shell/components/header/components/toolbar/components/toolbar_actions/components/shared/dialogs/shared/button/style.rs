use super::props::ButtonVariant;
use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "px-14",
        "py-6",
        "rounded-lg",
        "text-2xl",
        "whitespace-nowrap",
        "cursor-pointer",
        "select-none",
        "transition-all", "duration-fast",
    ],
}

states! {
    ButtonVariant,
    Primary => tw![
        "border",
        "border-warcraft-gold",
        "bg-panel-blue",
        "text-warcraft-gold",
        "text-shadow-drop",
        "hover:bg-panel-blue",
        "hover:shadow-glow",
    ],
    Secondary => tw![
        "border",
        "border-warcraft-blue",
        "bg-warcraft-bg-panel/70",
        "text-warcraft-text-secondary",
        "text-shadow-drop",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "hover:shadow-glow-soft",
    ],
}
