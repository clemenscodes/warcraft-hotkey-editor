use tw_macro::tw;

classes! {
    base: tw![
        "flex-1",
        "min-w-0",
        "w-full",
        "h-full",
        "pr-3.5",
        "py-0",
        "bg-transparent",
        "border-0",
        "text-base",
        "tracking-label",
        "placeholder:italic",
        "text-white",
        "placeholder:text-warcraft-text-primary/50",
        "kb-focus:outline-none",
        "[&::-webkit-search-cancel-button]:appearance-none",
    ],
    desktop: tw![
        "desktop:pr-4.25",
        "desktop:text-md",
    ],
    qhd: tw![
        "qhd:pr-6",
        "qhd:text-xl",
    ],
    uhd: tw![
        "uhd:pr-6.5",
        "uhd:text-2xl",
    ],
}
