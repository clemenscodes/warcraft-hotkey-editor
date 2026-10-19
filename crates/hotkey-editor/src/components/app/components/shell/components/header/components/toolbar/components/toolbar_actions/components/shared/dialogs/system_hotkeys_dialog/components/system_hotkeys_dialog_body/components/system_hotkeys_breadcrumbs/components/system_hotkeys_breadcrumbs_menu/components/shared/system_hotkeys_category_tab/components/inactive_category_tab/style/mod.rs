use tw_macro::tw;

classes! {
    base: tw![
        "uppercase",
        "tracking-heading",
        "text-2xl",
        "leading-none",
        "px-3",
        "py-1",
        "bg-transparent",
        "border-0",
        "cursor-pointer",
        "whitespace-nowrap",
        "text-shadow-drop",
        "transition-[color,text-shadow]",
        "hover:text-warcraft-gold",
        "hover:text-shadow-glow",
        "kb-focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:[--glow-color:var(--color-warcraft-highlight)]",
        "kb-focus:text-glow",
        "text-warcraft-gold/55",
    ],
    mobile: tw![
        "mobile:text-sm",
    ],
    tablet: tw![
        "tablet:text-base",
    ],
    laptop: tw![
        "laptop:text-md",
    ],
    desktop: tw![
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:px-4.25",
        "qhd:py-1.5",
        "qhd:text-xl",
    ],
    uhd: tw![
        "uhd:px-5",
        "uhd:py-1.75",
        "uhd:text-2xl",
    ],
}
