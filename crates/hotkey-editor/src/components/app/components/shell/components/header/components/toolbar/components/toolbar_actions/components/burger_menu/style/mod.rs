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
