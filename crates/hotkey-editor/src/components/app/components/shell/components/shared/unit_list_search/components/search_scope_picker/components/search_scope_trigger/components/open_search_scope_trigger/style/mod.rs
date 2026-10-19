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
    desktop: tw![
        "desktop:gap-1.25",
        "desktop:px-1.25",
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:gap-1.75",
        "qhd:px-1.75",
        "qhd:text-xl",
    ],
    uhd: tw![
        "uhd:gap-1.75",
        "uhd:px-1.75",
        "uhd:text-xl",
    ],
}
