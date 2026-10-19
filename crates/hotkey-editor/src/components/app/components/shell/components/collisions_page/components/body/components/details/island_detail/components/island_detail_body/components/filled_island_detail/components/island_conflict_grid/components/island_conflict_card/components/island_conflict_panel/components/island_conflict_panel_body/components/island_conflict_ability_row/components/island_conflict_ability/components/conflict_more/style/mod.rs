use tw_macro::tw;
classes! {
    base: tw![
        "p-0",
        "bg-transparent",
        "border-none",
        "cursor-pointer",
        "text-sm",
        "text-warcraft-text-muted",
        "underline",
        "underline-offset-2",
        "hover:text-warcraft-gold",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:text-[0.9em]",
    ],
    tablet: tw![
        "tablet:text-[0.9em]",
    ],
    laptop: tw![
        "laptop:text-[0.9em]",
    ],
}
