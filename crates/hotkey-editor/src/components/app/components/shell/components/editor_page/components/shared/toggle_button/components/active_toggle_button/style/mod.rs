use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-w-0",
        "px-6",
        "border",
        "rounded-panel",
        "text-xl",
        "uppercase",
        "tracking-caps",
        "whitespace-nowrap",
        "text-shadow-drop",
        "cursor-pointer",
        "transition-[border-color,color,box-shadow]",
        "duration-base",
        "bg-panel-gold-active",
        "border-warcraft-gold",
        "text-warcraft-gold",
        "shadow-glow",
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
