use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "gap-2",
        "text-xl",
        "leading-title",
        "min-w-0",
        "text-glow",
        "[--attribute-label-color:var(--color-warcraft-gold)]",
    ],
    mobile: tw![
        "mobile:text-2xl",
        "mobile:leading-heading",
    ],
}
