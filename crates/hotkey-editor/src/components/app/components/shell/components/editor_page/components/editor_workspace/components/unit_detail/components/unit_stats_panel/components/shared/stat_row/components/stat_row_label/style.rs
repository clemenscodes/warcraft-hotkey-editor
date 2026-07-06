use tw_macro::tw;
classes! {
    base: tw![
        "flex-[0_1_auto]",
        "min-w-0",
        "text-[inherit]",
        "text-warcraft-gold/90",
        "overflow-hidden",
        "text-ellipsis",
        "whitespace-nowrap",
        "group-data-[regen=true]:text-warcraft-gold/70",
        "group-data-[primary=true]:text-warcraft-gold",
        "group-data-[regen=true]:text-[clamp(1.3rem,0.85rem+0.32vw,1.75rem)]",
    ],
}
