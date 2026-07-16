use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-center",
        "gap-4",
        "py-8",
        "px-5",
        "bg-warcraft-bg-mid/50",
        "border",
        "border-warcraft-blue-deep",
        "rounded-card",
        "cursor-pointer",
        "hover:border-warcraft-gold",
        "hover:shadow-glow-soft",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
    ],
    mobile: tw![
        "mobile:gap-2",
        "mobile:py-4",
        "mobile:px-3",
    ],
}
