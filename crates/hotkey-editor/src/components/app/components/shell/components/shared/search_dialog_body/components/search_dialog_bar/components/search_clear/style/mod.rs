use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-center",
        "justify-center",
        "self-stretch",
        "shrink-0",
        "px-3",
        "border-0",
        "bg-transparent",
        "text-warcraft-text-secondary",
        "cursor-pointer",
        "transition-colors",
        "duration-base",
        "hover:text-warcraft-gold",
        "focus:outline-none",
        "kb-focus:text-white",
    ],
    desktop: tw![
        "desktop:px-3.5",
    ],
    qhd: tw![
        "qhd:px-5.25",
    ],
    uhd: tw![
        "uhd:px-5.5",
    ],
}
