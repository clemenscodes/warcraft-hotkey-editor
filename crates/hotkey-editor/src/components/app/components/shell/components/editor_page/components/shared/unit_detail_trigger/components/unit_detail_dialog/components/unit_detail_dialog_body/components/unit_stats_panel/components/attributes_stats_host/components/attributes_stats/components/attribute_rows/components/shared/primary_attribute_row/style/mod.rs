use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "items-baseline",
        "gap-[0.75em]",
        "leading-heading",
        "min-w-0",
        "text-glow",
        "[--attribute-label-color:var(--color-warcraft-gold)]",
    ],
    mobile: tw![
        "mobile:gap-[2.09cqi]",
    ],
}
