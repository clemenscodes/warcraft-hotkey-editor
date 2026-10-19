use tw_macro::tw;

classes! {
    base: tw![
        "flex",
        "items-center",
        "gap-2.5",
        "w-full",
        "h-10.5",
        "px-3",
        "rounded-panel",
        "border",
        "border-warcraft-gold/30",
        "bg-warcraft-bg-base",
        "text-warcraft-text-primary",
        "cursor-pointer",
        "transition-[border-color,box-shadow]",
        "duration-base",
        "hover:border-warcraft-gold/55",
        "focus:outline-none",
        "kb-focus:border-white",
    ],
    desktop: tw![
        "desktop:gap-3",
        "desktop:h-12.75",
        "desktop:px-3.5",
    ],
    qhd: tw![
        "qhd:gap-4.25",
        "qhd:h-18.25",
        "qhd:px-5.25",
    ],
    uhd: tw![
        "uhd:gap-4.75",
        "uhd:h-19.75",
        "uhd:px-5.5",
    ],
}
