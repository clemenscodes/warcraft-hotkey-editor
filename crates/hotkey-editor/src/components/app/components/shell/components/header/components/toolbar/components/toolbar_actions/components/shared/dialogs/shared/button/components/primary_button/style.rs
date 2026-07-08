use tw_macro::tw;

// The shared action-button chrome plus the primary weight: a gold-bordered blue
// panel with gold text that glows on hover.

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "px-14",
        "py-6",
        "rounded-card",
        "text-2xl",
        "whitespace-nowrap",
        "cursor-pointer",
        "select-none",
        "transition-all",
        "duration-fast",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
        "border",
        "border-warcraft-gold",
        "bg-panel-blue",
        "text-warcraft-gold",
        "text-shadow-drop",
        "hover:bg-panel-blue",
        "hover:shadow-glow",
    ],
}
