use tw_macro::tw;
// The idle crumb: the shared crumb chrome values muted, lightening on hover, publishing
// dimmed count opacity. Shared values with the active sibling.
classes! {
    base: tw![
        "m-0",
        "py-1", "px-0.5",
        "bg-transparent",
        "border-none",
        "cursor-pointer",
        "inline-flex",
        "items-center",
        "gap-2",
        "text-warcraft-text-muted",
        "[--count-opacity:0.8]",
        "transition-colors",
        "duration-fast",
        "hover:text-warcraft-text-primary",
        "kb-focus:outline-none",
        "kb-focus:shadow-focus",
    ],
    mobile: tw!["mobile:flex-none", "mobile:snap-start"],
    tablet: tw!["tablet:flex-none", "tablet:snap-start"],
}
