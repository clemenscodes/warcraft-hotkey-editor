use tw_macro::tw;
classes! {
    base: tw![
        "block",
        "h-[2.4rem]",
        "w-auto",
        "flex-none",
        "[filter:drop-shadow(0_1px_0_color-mix(in_oklab,var(--color-warcraft-shadow)_70%,transparent))]",
    ],
    mobile: tw!["mobile:w-[2rem]"],
    tablet: tw!["tablet:w-[2.75rem]"],
}
