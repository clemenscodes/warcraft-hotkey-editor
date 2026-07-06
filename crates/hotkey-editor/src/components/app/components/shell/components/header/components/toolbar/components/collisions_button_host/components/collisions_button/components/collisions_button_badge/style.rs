use tw_macro::tw;
// The count badge scales with the button: its corner offsets and font size are cqi
// fractions of the button box, so it holds its position and proportion at any size.

classes! {
    base: tw![
        "absolute",
        "top-[8cqi]",
        "right-[9cqi]",
        "font-bold",
        "leading-none",
        "text-[length:20cqi]",
        "text-warcraft-gold",
        "pointer-events-none",
        "[text-shadow:1.25cqi_1.25cqi_0_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent),-1.25cqi_1.25cqi_0_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent),1.25cqi_-1.25cqi_0_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent),-1.25cqi_-1.25cqi_0_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent),0_0_3.75cqi_color-mix(in_oklab,var(--color-warcraft-shadow)_95%,transparent)]",
    ],
}
