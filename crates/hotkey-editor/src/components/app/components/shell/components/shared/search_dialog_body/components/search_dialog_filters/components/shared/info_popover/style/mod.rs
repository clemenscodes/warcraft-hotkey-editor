use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "flex-none",
        "w-4",
        "h-4",
        "p-0",
        "rounded-full",
        "border",
        "border-warcraft-gold/45",
        "bg-transparent",
        "text-warcraft-text-secondary",
        "text-2xs",
        "font-bold",
        "leading-none",
        "cursor-pointer",
        "transition-[color,border-color]",
        "duration-base",
        "hover:text-warcraft-gold",
        "hover:border-warcraft-gold",
        "focus:outline-none",
        "kb-focus:text-white",
        "kb-focus:border-white",
    ],
    desktop: tw![
        "desktop:w-4.75",
        "desktop:h-4.75",
        "desktop:text-xs",
    ],
    qhd: tw![
        "qhd:w-7",
        "qhd:h-7",
        "qhd:text-base",
    ],
    uhd: tw![
        "uhd:w-7.5",
        "uhd:h-7.5",
        "uhd:text-md",
    ],
}
