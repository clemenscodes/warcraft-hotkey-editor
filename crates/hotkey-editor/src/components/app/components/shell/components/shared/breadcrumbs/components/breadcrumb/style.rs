use tw_macro::tw;
classes! {
    base: tw![
        "group",
        "m-0",
        "py-1", "px-0.5",
        "bg-transparent",
        "border-none",
        "cursor-pointer",
        "inline-flex",
        "items-center",
        "gap-2",
        "text-warcraft-text-muted",
        "transition-colors",
        "duration-fast",
        "hover:text-warcraft-text-primary",
        "data-[active=true]:text-warcraft-gold",
        "data-[active=true]:text-shadow-drop",
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
