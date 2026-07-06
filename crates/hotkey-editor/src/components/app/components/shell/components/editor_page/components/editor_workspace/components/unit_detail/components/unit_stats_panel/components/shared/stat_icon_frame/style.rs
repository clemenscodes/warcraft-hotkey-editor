use tw_macro::tw;
classes! {
    base: tw![
        "flex-[0_0_auto]",
        "self-start",
        "w-[clamp(4rem,2.7vw+1.75rem,5.75rem)]",
        "h-[clamp(4rem,2.7vw+1.75rem,5.75rem)]",
        "[filter:drop-shadow(0_1px_2px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent))]",
    ],
    mobile: tw![
        "mobile:w-[5rem]",
        "mobile:h-[5rem]",
    ],
}
