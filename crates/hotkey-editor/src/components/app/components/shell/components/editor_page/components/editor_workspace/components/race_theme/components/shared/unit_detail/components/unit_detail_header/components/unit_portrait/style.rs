use tw_macro::tw;
classes! {
    base: tw![
        "size-24",
        "[image-rendering:auto]",
        "border-2",
        "border-warcraft-blue",
        "rounded-control",
        "shadow-raised",
        "object-cover",
        "bg-warcraft-bg-panel/70",
        "text-transparent",
        "text-[0]",
        "leading-0",
    ],
    mobile: tw![
        "mobile:shrink-0",
        "mobile:self-start",
    ],
    tablet: tw![
        "tablet:shrink-0",
        "tablet:self-start",
        "tablet:size-28",
    ],
}
