use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "items-baseline",
        "gap-[2.09cqi]",
        "text-xl",
        "leading-title",
        "min-w-0",
        "text-glow",
        "[--attribute-label-color:var(--color-warcraft-gold)]",
    ],
    mobile: tw![
        "mobile:text-sm",
        "mobile:leading-heading",
    ],
}
