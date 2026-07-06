use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "w-[3.5rem]",
        "h-[3.5rem]",
        "text-warcraft-gold",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
        "[filter:drop-shadow(0_0_10px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent))]",
    ],
}
