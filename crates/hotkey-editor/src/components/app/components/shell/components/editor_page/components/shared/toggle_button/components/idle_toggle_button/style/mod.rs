use tw_macro::tw;

classes! {
    base: tw![
        "flex-1",
        "px-6",
        "bg-panel-gold-resting",
        "border",
        "border-warcraft-gold-border",
        "rounded-panel",
        "text-warcraft-text-secondary",
        "text-xl",
        "uppercase",
        "tracking-caps",
        "whitespace-nowrap",
        "text-shadow-drop",
        "cursor-pointer",
        "transition-[border-color,color,box-shadow]",
        "duration-base",
        "hover:border-warcraft-gold",
        "hover:text-warcraft-gold",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:text-base",
        "mobile:px-2.5",
    ],
    tablet: tw![
        "tablet:text-md",
        "tablet:px-4",
    ],
    laptop: tw![
        "laptop:text-md",
        "laptop:px-4",
    ],
    desktop: tw![
        "desktop:text-md",
        "desktop:px-4",
    ],
}
