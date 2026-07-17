use tw_macro::tw;

classes! {
    base: tw![
        "block",
        "w-full",
        "py-1.5",
        "px-3",
        "border-none",
        "rounded-tile",
        "text-left",
        "cursor-pointer",
        "whitespace-nowrap",
        "transition-[background,color]",
        "duration-fast",
        "kb-focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
        "[background:linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold)_25%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-gold-dark)_70%,transparent)_100%)]",
        "text-warcraft-gold",
    ],
}
