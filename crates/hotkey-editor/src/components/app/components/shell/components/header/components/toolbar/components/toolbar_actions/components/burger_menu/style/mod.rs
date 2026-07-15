use tw_macro::tw;

classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "shrink-0",
        "@container",
        "h-full",
        "aspect-square",
        "p-0",
        "bg-panel-gold-resting",
        "border",
        "border-warcraft-gold-border",
        "rounded-tile",
        "text-warcraft-text-secondary",
        "cursor-pointer",
        "transition-[border-color,color,background,box-shadow]",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "hover:bg-panel-gold-active",
        "hover:shadow-glow",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:w-9",
        "mobile:h-9",
    ],
    tablet: tw![
        "tablet:w-9",
        "tablet:h-9",
    ],
    desktop: tw![
        "desktop:hidden",
    ],
    qhd: tw![
        "qhd:hidden",
    ],
    uhd: tw![
        "uhd:hidden",
    ],
}
