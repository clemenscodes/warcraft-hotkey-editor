use super::state::BurgerItemState;
use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-3",
        "w-full",
        "min-h-10",
        "py-2",
        "px-3",
        "bg-panel-gold-resting",
        "border",
        "border-warcraft-gold-border",
        "rounded-card",
        "text-warcraft-text-secondary",
        "text-sm",
        "tracking-label",
        "text-left",
        "cursor-pointer",
        "transition-[border-color,color,background,box-shadow]",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]", "kb-focus:shadow-focus",
    ],
}

states! {
    BurgerItemState,
    Idle => tw![],
    Active => tw![
        "bg-panel-gold-active",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-ring",
    ],
    Primary => tw![
        "bg-panel-gold-resting",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow",
        "hover:bg-panel-gold",
        "hover:shadow-glow-strong",
    ],
}
