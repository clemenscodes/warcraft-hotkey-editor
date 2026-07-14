use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "justify-between",
        "gap-2",
        "px-2",
        "py-1.5",
        "rounded-control",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
        "min-w-0",
        "[--matchup-color:var(--color-warcraft-text-secondary)]",
    ],
    mobile: tw![
        "mobile:text-xl",
        "mobile:px-2.5",
        "mobile:py-1.5",
    ],
}
