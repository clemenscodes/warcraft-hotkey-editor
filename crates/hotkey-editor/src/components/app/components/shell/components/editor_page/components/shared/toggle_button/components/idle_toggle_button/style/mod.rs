use tw_macro::tw;

classes! {
    base: tw![
        "px-2",
        "bg-panel-gold-resting",
        "border",
        "border-(--race-color,var(--color-warcraft-gold-border))/70",
        "rounded-panel",
        "text-warcraft-text-secondary",
        "text-sm",
        "whitespace-nowrap",
        "min-h-11",
        "flex",
        "items-center",
        "justify-center",
        "text-shadow-drop",
        "cursor-pointer",
        "transition-[border-color,color,box-shadow]",
        "duration-base",
        "hover:border-(--race-color,var(--color-warcraft-gold))",
        "hover:text-(--race-color,var(--color-warcraft-gold))",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:px-2",
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
