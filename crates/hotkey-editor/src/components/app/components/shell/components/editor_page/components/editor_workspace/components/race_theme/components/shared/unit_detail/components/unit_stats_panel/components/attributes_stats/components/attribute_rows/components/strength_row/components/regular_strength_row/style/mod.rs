use tw_macro::tw;
classes! {
    base: tw![
        "@container",
        "flex",
        "items-baseline",
        "gap-[2.09cqi]",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
        "min-w-0",
        "[--attribute-label-color:color-mix(in_oklab,var(--color-warcraft-gold)_90%,transparent)]",
    ],
    mobile: tw![
        "mobile:text-2xl",
        "mobile:leading-heading",
    ],
}
