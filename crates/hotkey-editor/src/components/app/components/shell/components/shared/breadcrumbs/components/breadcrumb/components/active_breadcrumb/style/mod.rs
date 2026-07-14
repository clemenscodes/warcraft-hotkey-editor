use tw_macro::tw;
classes! {
    base: tw![
        "py-1",
        "px-0.5",
        "bg-transparent",
        "border-none",
        "cursor-pointer",
        "inline-flex",
        "items-center",
        "gap-2",
        "text-warcraft-gold",
        "text-shadow-drop",
        "[--count-opacity:1]",
        "transition-colors",
        "duration-fast",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:snap-start",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:snap-start",
    ],
}
