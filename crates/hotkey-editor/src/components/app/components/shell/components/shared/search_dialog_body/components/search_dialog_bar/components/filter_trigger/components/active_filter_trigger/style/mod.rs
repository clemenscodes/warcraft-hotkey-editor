use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "flex",
        "items-center",
        "justify-center",
        "gap-1.5",
        "self-stretch",
        "px-3",
        "border-0",
        "border-r",
        "border-warcraft-gold/30",
        "bg-warcraft-gold/15",
        "text-warcraft-gold",
        "cursor-pointer",
        "transition-[background,color]",
        "duration-base",
        "focus:outline-none",
        "kb-focus:text-white",
    ],
    desktop: tw![
        "desktop:gap-1.75",
        "desktop:px-3.5",
    ],
    qhd: tw![
        "qhd:gap-2.5",
        "qhd:px-5.25",
    ],
    uhd: tw![
        "uhd:gap-2.75",
        "uhd:px-5.5",
    ],
}
