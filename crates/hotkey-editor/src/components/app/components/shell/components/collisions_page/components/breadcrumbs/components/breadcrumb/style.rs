use tw_macro::tw;
classes! {
    base: tw![
        "group",
        "m-0",
        "p-[0.25rem_0.15rem]",
        "bg-transparent",
        "border-none",
        "cursor-pointer",
        "inline-flex",
        "items-center",
        "gap-[0.45rem]",
        "text-warcraft-text-muted",
        "transition-colors",
        "duration-[120ms]",
        "hover:text-warcraft-text-primary",
        "data-[active=true]:text-warcraft-gold",
        "data-[active=true]:text-shadow-drop",
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
