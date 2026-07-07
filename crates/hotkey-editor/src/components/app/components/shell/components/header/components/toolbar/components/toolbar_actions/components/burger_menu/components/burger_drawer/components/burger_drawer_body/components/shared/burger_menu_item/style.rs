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
        "tracking-[0.05em]",
        "text-left",
        "cursor-pointer",
        "[transition:border-color_0.15s_ease,color_0.15s_ease,background_0.15s_ease,box-shadow_0.15s_ease]",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow-12",
        "focus:outline-none",
        "focus-visible:border-white",
        "focus-visible:text-white",
        "focus-visible:shadow-ring-hl",
    ],
}

states! {
    BurgerItemState,
    Idle => tw![],
    Active => tw![
        "[background:linear-gradient(180deg,color-mix(in_oklab,var(--color-warcraft-gold)_22%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_60%,transparent)_100%)]",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "[box-shadow:var(--shadow-ring-gold)]",
    ],
    Primary => tw![
        "bg-panel-gold-diag-85",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow-22",
        "hover:bg-panel-gold-diag-22",
        "hover:[box-shadow:var(--shadow-glow-gold-inset)]",
    ],
}
