use tw_macro::tw;

classes! {
    base: tw![
        "max-w-full",
        "overflow-hidden",
        "whitespace-nowrap",
        "text-ellipsis",
        "font-normal",
        "text-2xl",
        "leading-title",
        "text-(--race-color,var(--color-warcraft-gold))",
        "text-shadow-drop",
    ],
    mobile: tw![
        "mobile:text-[1.25em]",
        "mobile:break-normal",
    ],
    tablet: tw![
        "tablet:text-sm",
        "tablet:break-normal",
    ],
}
