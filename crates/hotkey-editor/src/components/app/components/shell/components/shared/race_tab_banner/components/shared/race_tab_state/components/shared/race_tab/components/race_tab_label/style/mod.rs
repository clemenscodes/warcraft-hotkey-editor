use tw_macro::tw;

classes! {
    base: tw![
        "relative",
        "w-full",
        "min-w-0",
        "whitespace-nowrap",
        "text-center",
        "text-(--label-color,var(--color-white))",
    ],
    mobile: tw![
        "mobile:tracking-snug",
    ],
    tablet: tw![
        "tablet:tracking-snug",
    ],
}
