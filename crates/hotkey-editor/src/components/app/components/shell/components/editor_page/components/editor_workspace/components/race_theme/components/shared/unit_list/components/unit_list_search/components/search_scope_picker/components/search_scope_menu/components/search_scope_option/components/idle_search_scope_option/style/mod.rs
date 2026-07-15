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
        "bg-transparent",
        "text-warcraft-text-secondary",
        "hover:bg-warcraft-gold/12",
        "hover:text-warcraft-gold",
    ],
}
