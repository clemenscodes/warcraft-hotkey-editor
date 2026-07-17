use tw_macro::tw;

classes! {
    base: tw![
        "@container",
        "flex-1",
        "min-w-0",
        "px-(--toggle-pad,1.5rem)",
        "border",
        "rounded-panel",
        "text-(length:--toggle-font,var(--text-xl))",
        "uppercase",
        "tracking-caps",
        "whitespace-nowrap",
        "overflow-hidden",
        "text-ellipsis",
        "text-shadow-drop",
        "cursor-pointer",
        "transition-[border-color,color,box-shadow]",
        "duration-base",
        "bg-panel-gold-active",
        "border-(--race-color,var(--color-warcraft-gold))",
        "text-(--race-color,var(--color-warcraft-gold))",
        "shadow-glow",
        "focus:outline-none",
        "kb-focus:border-white",
        "kb-focus:text-white",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:text-(length:--toggle-font,var(--text-base))",
        "mobile:px-(--toggle-pad,0.625rem)",
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
