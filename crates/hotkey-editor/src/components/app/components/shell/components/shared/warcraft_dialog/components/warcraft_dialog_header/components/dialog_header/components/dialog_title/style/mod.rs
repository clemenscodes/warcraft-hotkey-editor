use tw_macro::tw;
classes! {
    base: tw![
        "text-3xl",
        "leading-none",
        "uppercase",
        "tracking-heading",
        "text-warcraft-gold",
        "text-glow",
    ],
    mobile: tw![
        "mobile:min-w-0",
        "mobile:max-w-full",
        "mobile:overflow-hidden",
        "mobile:text-sm",
        "mobile:leading-none",
        "mobile:text-ellipsis",
        "mobile:whitespace-nowrap",
    ],
    tablet: tw![
        "tablet:min-w-0",
        "tablet:max-w-full",
        "tablet:overflow-hidden",
        "tablet:text-sm",
        "tablet:leading-none",
        "tablet:text-ellipsis",
        "tablet:whitespace-nowrap",
    ],
}
