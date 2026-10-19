use tw_macro::tw;

classes! {
    base: tw![
        "group",
        "relative",
        "flex-none",
        "w-10.5",
        "h-6",
        "p-0",
        "rounded-full",
        "border",
        "border-warcraft-gold/30",
        "bg-warcraft-bg-base",
        "cursor-pointer",
        "transition-[background,border-color]",
        "duration-base",
        "aria-pressed:border-warcraft-gold",
        "aria-pressed:bg-warcraft-gold/30",
        "focus:outline-none",
        "kb-focus:border-white",
    ],
    desktop: tw![
        "desktop:w-12.75",
        "desktop:h-7.25",
    ],
    qhd: tw![
        "qhd:w-18.25",
        "qhd:h-10.5",
    ],
    uhd: tw![
        "uhd:w-19.75",
        "uhd:h-11.25",
    ],
}
