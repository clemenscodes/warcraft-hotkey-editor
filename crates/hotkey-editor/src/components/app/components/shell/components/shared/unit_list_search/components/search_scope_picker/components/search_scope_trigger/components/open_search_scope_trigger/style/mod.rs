use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-1",
        "px-1",
        "whitespace-nowrap",
        "bg-warcraft-gold/10",
        "border-none",
        "text-white",
        "text-sm",
        "uppercase",
        "tracking-caps",
        "cursor-pointer",
        "transition-[color]",
        "kb-focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:[--focus-color:var(--color-warcraft-highlight)]",
        "kb-focus:shadow-focus",
    ],
}
