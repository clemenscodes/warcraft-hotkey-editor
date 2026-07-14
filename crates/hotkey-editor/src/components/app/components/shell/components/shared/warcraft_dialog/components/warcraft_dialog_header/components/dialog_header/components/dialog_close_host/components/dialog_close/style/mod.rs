use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "size-full",
        "text-[50cqi]",
        "flex",
        "items-center",
        "justify-center",
        "bg-transparent",
        "border-0",
        "cursor-pointer",
        "text-warcraft-text-secondary",
        "text-shadow-drop",
        "transition-[color,text-shadow]",
        "duration-base",
        "hover:text-warcraft-gold",
        "hover:text-shadow-glow",
        "focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:[--glow-color:var(--color-warcraft-highlight)]",
        "kb-focus:text-glow",
    ],
}
